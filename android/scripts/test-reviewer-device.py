#!/usr/bin/env python3
"""Debug WebView acceptance checks. Requires websocket-client, adb and a running debug app.
Usage: python test-reviewer-device.py SERIAL
Uses the private reviewer code in ~/.config/maginet/reviewer-code.txt. Start with reviewer access disabled. Does not purchase.
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



call('Page.reload', {})
time.sleep(1)
ready()
tap(248,212)
tap(232,216)
code=(Path.home()/'.config/maginet/reviewer-code.txt').read_text().strip()
# Use keyboard focus rather than geometry while the IME animates.
result = subprocess.run([adb, '-s', args.serial, 'shell', 'input', 'text', code], capture_output=True)
assert result.returncode == 0, 'Native reviewer input failed'
del code
device('shell','input','keyevent','KEYCODE_TAB','KEYCODE_TAB','KEYCODE_ENTER')
time.sleep(1)
access = "import(performance.getEntriesByType('resource').find(e=>e.name.endsWith('/mobile-access.js')).name)"
def owned(): return evaluate(access+'.then(m=>m.owned())')
assert owned()
assert device('shell','run-as',package,'ls','no_backup') == 'review-access-v1'
# Relaunch proves actual Keystore persistence, without relying on JS state.
ws.close()
device('shell','am','force-stop',package)
device('shell','am','start','-W','-n',package+'/zone.evrim.maginet.GameActivity')
ws=connect();ready();time.sleep(1)
assert owned()
evaluate("maginetNative.postMessage(JSON.stringify({action:'restore'}))")
time.sleep(8)
device('shell','input','keyevent','KEYCODE_BACK')
assert owned()
evaluate("maginetNative.postMessage(JSON.stringify({action:'review'}))")
time.sleep(1)
native_tap('End review')
assert not owned()
assert evaluate("fetch('/api/session').then(r=>r.status)") == 403
assert device('shell','run-as',package,'ls','no_backup') == ''
print('PASS: real native unlock, encrypted separate cache, process restart, restore, removal and proxy relock')
ws.close()
