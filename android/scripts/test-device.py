#!/usr/bin/env python3
"""Debug WebView acceptance checks. Requires websocket-client, adb and a running debug app.
Usage: python test-device.py SERIAL [--install APK]
Does not simulate ownership or charge a purchase. Uses a disposable localStorage test key.
"""
import argparse
import json
import os
import re
import socket
import xml.etree.ElementTree as ET
from pathlib import Path
import subprocess
import time
import urllib.request
import websocket

parser = argparse.ArgumentParser()
parser.add_argument('serial')
parser.add_argument('--install', metavar='APK')
args = parser.parse_args()
adb = str(Path(os.environ.get('ANDROID_HOME', str(Path.home() / 'Library/Android/sdk'))) / 'platform-tools/adb')
package = 'zone.evrim.maginet.debug'

def device(*args):
    return subprocess.check_output([adb, '-s', parser_args.serial, *args], text=True).strip()
parser_args = args

def connect():
    for _ in range(30):
        try:
            pid = device('shell', 'pidof', package)
            device('forward', 'tcp:19222', 'localabstract:webview_devtools_remote_' + pid)
            pages = json.load(urllib.request.urlopen('http://127.0.0.1:19222/json'))
            return websocket.create_connection(pages[0]['webSocketDebuggerUrl'], suppress_origin=True, timeout=20)
        except Exception:
            time.sleep(.2)
    raise RuntimeError('WebView debugger unavailable')

ws = connect()
sequence = 0

def call(method, params):
    global sequence
    sequence += 1
    ws.send(json.dumps({'id': sequence, 'method': method, 'params': params}))
    while True:
        result = json.loads(ws.recv())
        if result.get('id') == sequence:
            assert 'error' not in result, result
            return result['result']

def evaluate(source):
    result = call('Runtime.evaluate', {'expression': source, 'awaitPromise': True, 'returnByValue': True})
    assert 'exceptionDetails' not in result, result
    return result['result'].get('value')

def ready():
    for _ in range(50):
        if evaluate('!!document.querySelector("canvas")'): return
        time.sleep(.2)
    raise AssertionError('Bundled Wasm did not create a canvas')

call('Page.reload', {})
time.sleep(.7)
ready()
assert evaluate('window.maginetSafeLeft === window.maginetSafeRight'), 'Landscape side insets must be symmetric'
assert evaluate('location.origin') == 'http://127.0.0.1:18743'
raster = evaluate('''new Promise(resolve => {
 const original=CanvasRenderingContext2D.prototype.drawImage;
 CanvasRenderingContext2D.prototype.drawImage=function(source,...args) {
   const result=original.call(this,source,...args);
   if(this.canvas.id==='game-canvas') {
     CanvasRenderingContext2D.prototype.drawImage=original;
     resolve({height:source.height, scale:args[3]/source.height, smoothing:this.imageSmoothingEnabled});
   }
   return result;
 };
})''')
assert raster['height'] == 272 and not raster['smoothing'], raster
assert evaluate("Math.abs(document.querySelector('canvas').getBoundingClientRect().height-innerHeight)<1"), raster
fixture = (Path(__file__).resolve().parents[2] / 'ios/Tests/ai-fixture.json').read_text()
worker = evaluate('''new Promise((resolve,reject)=>{
 const worker=new Worker('/static/js/ai-worker.js',{type:'module'});
 const timer=setTimeout(()=>{worker.terminate();reject('worker timeout')},10000);
 worker.onmessage=e=>{clearTimeout(timer);worker.terminate();resolve(e.data)};
 worker.onerror=e=>{clearTimeout(timer);worker.terminate();reject(e.message)};
 worker.postMessage({snapshot:''' + json.dumps(fixture) + ''',difficulty:'Easy',seed:'1',id:7,revision:3});
})''')
assert worker.get('selected') is not None and not worker.get('failed'), worker
assert evaluate("fetch('/api/session').then(r=>r.status)") == 403
assert evaluate("fetch('/api/admin').then(r=>r.status)") == 403
assert evaluate("fetch('/static/png/atlas.png?v=8').then(r=>r.status)") == 200
assert evaluate("fetch('/%2e%2e/Cargo.toml').then(r=>r.status)") in [403,404]
evaluate("maginetNative.postMessage('not json');maginetNative.postMessage(JSON.stringify({action:'state',owned:true}));maginetNative.postMessage(JSON.stringify({action:'grant'}))")
assert evaluate("fetch('/api/session').then(r=>r.status)") == 403
# Even forged JS ownership does not authorize the native proxy.
evaluate("window.dispatchEvent(new CustomEvent('maginet-access',{detail:true}))")
assert evaluate("fetch('/api/session').then(r=>r.status)") == 403
evaluate("maginetNative.postMessage(JSON.stringify({action:'state'}))")
access = "import(performance.getEntriesByType('resource').find(e=>e.name.endsWith('/mobile-access.js')).name)"
for _ in range(30):
    if evaluate(access + '.then(m=>m.owned())') is False: break
    time.sleep(.1)
else: raise AssertionError('Native state did not restore locked access')
# HTTP parsing restrictions, exercised against the actual native listener.
device('forward', 'tcp:19223', 'tcp:18743')
try:
    def http_status(request):
        with socket.create_connection(('127.0.0.1',19223),timeout=5) as sock:
            sock.sendall(request.encode())
            return int(sock.recv(4096).split(b' ')[1])
    assert http_status('GET / HTTP/1.1\r\nHost: evil.example\r\n\r\n') == 400
    assert http_status('GET / HTTP/1.1\r\nHost: 127.0.0.1:18743\r\nHost: 127.0.0.1:18743\r\n\r\n') == 400
    assert http_status('POST /api/lobby/create HTTP/1.1\r\nHost: 127.0.0.1:18743\r\nTransfer-Encoding: chunked\r\n\r\n') == 400
    assert http_status('POST /api/lobby/create HTTP/1.1\r\nHost: 127.0.0.1:18743\r\nContent-Length: 1000001\r\n\r\n') == 413
finally:
    device('forward','--remove','tcp:19223')
# Real touch injection: translate logical game coordinates through WebView and canvas bounds.
def tap(x, y):
    geometry = evaluate("""(()=>{
        const c=document.querySelector('canvas').getBoundingClientRect(), scale=c.height/272;
        const width=Math.ceil(innerWidth/scale), left=(window.maginetSafeLeft||0)/scale;
        const right=(window.maginetSafeRight||0)/scale;
        const padding=Math.round(Math.min(Math.max((width-256)/2,left+72),Math.max(width-right-320,left+72)));
        return {scale,padding,top:Math.floor(Math.max(0,Math.min(8,16-(window.maginetSafeBottom||0)/scale))),dpr:devicePixelRatio};
    })()""")
    pages = json.load(urllib.request.urlopen('http://127.0.0.1:19222/json'))
    frame = json.loads(pages[0]['description'])
    px = frame['screenX'] + (geometry['padding'] + x) * geometry['scale'] * geometry['dpr']
    py = frame['screenY'] + (geometry['top'] + y) * geometry['scale'] * geometry['dpr']
    device('shell', 'input', 'tap', str(round(px)), str(round(py)))
    time.sleep(.4)

def ui():
    device('shell', 'uiautomator', 'dump', '/sdcard/maginet-test-ui.xml')
    return ET.fromstring(device('shell', 'cat', '/sdcard/maginet-test-ui.xml'))

def native_text(text):
    return any(n.get('text', '').casefold() == text.casefold() for n in ui().iter('node'))

def native_tap(text):
    node = next(n for n in ui().iter('node') if n.get('text', '').casefold() == text.casefold())
    x1,y1,x2,y2=map(int,re.findall(r'\d+',node.get('bounds')))
    device('shell','input','tap',str((x1+x2)//2),str((y1+y2)//2))
    time.sleep(.3)

# A same-origin child frame must not gain access to native UI actions.
frame_bridge = evaluate("""new Promise(resolve=>{
 const frame=document.createElement('iframe');frame.srcdoc='';
 frame.onload=()=>{const available=!!frame.contentWindow.maginetNative;
   if(available) frame.contentWindow.maginetNative.postMessage(JSON.stringify({action:'purchase'}));
   frame.remove();resolve(available);};
 document.body.appendChild(frame);
})""")
assert not native_text('Purchases are not configured in this build.')
evaluate("location.href='https://example.com/'")
time.sleep(.3)
assert evaluate('location.origin') == 'http://127.0.0.1:18743'
tap(248,212)  # Settings
# Short Android tap must reach the native purchase gate.
tap(232,172)
assert native_text('Purchases are not configured in this build.')
device('shell','input','keyevent','KEYCODE_BACK')
assert not native_text('Leave Maginet?')
tap(72,248)  # Settings back
tap(248,144)  # Editor remains free
tap(276,212)  # Native level-code keyboard
assert native_text('Level code')
native_tap('Done')
assert evaluate("document.activeElement.id !== 'text-input'")
device('shell','input','keyevent','KEYCODE_BACK')
assert native_text('Leave Maginet?')
native_tap('Keep playing')
evaluate("localStorage.setItem('android-verification-progress','kept')")
# Real Android lifecycle callbacks and timer suspension.
device('shell', 'input', 'keyevent', 'KEYCODE_HOME')
time.sleep(.5)
assert evaluate(access + '.then(m=>m.inactive())') is True
device('shell', 'am', 'start', '-W', '-n', package + '/zone.evrim.maginet.GameActivity')
time.sleep(.5)
assert evaluate(access + '.then(m=>m.inactive())') is False
ws.close()
if args.install:
    device('install', '-r', str(Path(args.install).resolve()))
else:
    device('shell', 'am', 'force-stop', package)
device('shell', 'am', 'start', '-W', '-n', package + '/zone.evrim.maginet.GameActivity')
ws = connect(); ready()
assert evaluate("localStorage.getItem('android-verification-progress')") == 'kept'
evaluate("localStorage.removeItem('android-verification-progress')")
print(json.dumps({'result':'PASS','raster':raster,'workerSelected':worker['selected'],'persistence':'APK upgrade' if args.install else 'process restart'}))
ws.close()
device('forward','--remove','tcp:19222')
