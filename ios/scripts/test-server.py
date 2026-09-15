#!/usr/bin/env python3
"""Exercise the unchanged multiplayer protocol against a disposable local server."""
import json
import os
import pathlib
import subprocess
import socket
import tempfile
import time
import urllib.request

root = pathlib.Path(__file__).resolve().parents[2]
subprocess.run(['cargo', 'build', '--locked', '-p', 'server'], cwd=root, check=True)

def request(path, payload=None):
    data = None if payload is None else json.dumps(payload).encode()
    req = urllib.request.Request(f'http://127.0.0.1:{port}' + path, data=data,
                                 headers={'Content-Type': 'application/json'})
    with urllib.request.urlopen(req, timeout=3) as response:
        return json.load(response)

# Choose an unused port rather than touching a developer's running server.
with socket.socket() as probe:
    probe.bind(('127.0.0.1', 0))
    port = probe.getsockname()[1]

with tempfile.TemporaryDirectory(prefix='maginet-online-') as directory:
    server = subprocess.Popen([str(root / 'target/debug/server')], cwd=directory,
                              stdout=subprocess.DEVNULL, stderr=subprocess.PIPE,
                              env={**os.environ, "MAGINET_PORT": str(port)})
    try:
        for _ in range(100):
            if server.poll() is not None:
                raise RuntimeError('Disposable test server failed to start')
            try:
                a = request('/session')['session_id']
                break
            except OSError:
                time.sleep(.05)
        b = request('/session')['session_id']
        lobby = request('/lobby/create', {'lobby_settings': {
            'lobby_sort': {'Online': 0}, 'loadout_method': 'Default',
            'seed': 42, 'can_stalemate': True}})['Lobby']
        code = lobby['settings']['lobby_sort']['Online']
        for player in [a, b]:
            assert request(f'/lobby/{code}/ready', {'session_id': player}) == 'Ok'
        lobby = request(f'/lobby/{code}/state')['Lobby']
        assert len(lobby['players']) == 2
        turn = lobby['game']['available_turns'][0]
        assert request(f'/lobby/{code}/act', {'session_id': a, 'message': {'Turn': turn}}) == 'Ok'
        assert request(f'/lobby/{code}/turns/0', {'session_id': b})['Turns'] == [turn]
        for player in [a, b]:
            assert request(f'/lobby/{code}/rematch', {'session_id': player}) == 'Ok'
        assert request(f'/lobby/{code}/state')['Lobby']['game']['turns'] == []
        assert 'LobbyError' in request('/lobby/0/state')
        print('PASS: sessions, lobby creation, join, move synchronization, rematch, missing lobby')
    finally:
        server.terminate()
        server.wait(timeout=5)
