#!/usr/bin/env python3
"""Exercise deployment orchestration with fake builders/SteamCMD; never upload."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

REPO = Path(__file__).resolve().parents[1]

# Fake external commands emit identifiable payloads, so the assertions inspect
# the resulting depots as well as whether/when SteamCMD was invoked.
FAKE_TOOL = r'''#!/usr/bin/env python3
import json, os, pathlib, shutil, sys
tool = pathlib.Path(sys.argv[0]).name
args = sys.argv[1:]
with open(os.environ['CALL_LOG'], 'a') as log:
    log.write(json.dumps({'tool': tool, 'args': args, 'home': os.environ['HOME']}) + '\n')
if tool == 'wasm-pack':
    features = args[args.index('--features') + 1]
    if features == 'deploy,demo' and os.environ.get('FAIL_DEMO_BUILD'):
        sys.exit(4)
    out = pathlib.Path(args[args.index('--out-dir') + 1])
    out.mkdir(parents=True)
    (out / 'maginet_bg.wasm').write_text(features)
    (out / 'maginet.js').write_text('built javascript')
elif tool == 'electron-packager':
    source, name = pathlib.Path(args[0]), args[1]
    out = pathlib.Path(next(a.split('=', 1)[1] for a in args if a.startswith('--out=')))
    platform = next(a.split('=', 1)[1] for a in args if a.startswith('--platform='))
    arch = next(a.split('=', 1)[1] for a in args if a.startswith('--arch='))
    package = out / f'{name}-{platform}-{arch}'
    if platform == 'win32':
        app = package / 'resources/app'
        binary = package / f'{name}.exe'
    else:
        app = package / f'{name}.app/Contents/Resources/app'
        binary = package / f'{name}.app/Contents/MacOS/{name}'
    shutil.copytree(source, app)
    binary.parent.mkdir(parents=True, exist_ok=True)
    binary.write_text('executable')
    binary.chmod(0o755)
elif tool == 'steamcmd':
    if os.environ.get('FAIL_STEAM_EXIT'):
        sys.exit(7)
    print('Successfully finished AppID 2441960 build (BuildID 123)')
    if not os.environ.get('FAIL_DEMO_UPLOAD'):
        print('Successfully finished AppID 2529900 build (BuildID 456)')
'''


class SteamDeployTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='maginet steam test ')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / 'checkout with spaces'
        self.root.mkdir()
        shutil.copy2(REPO / 'deploy-steam.sh', self.root)
        shutil.copytree(REPO / 'platforms/steam/app', self.root / 'platforms/steam/app')
        shutil.copy2(REPO / 'platforms/steam/package.json', self.root / 'platforms/steam')
        for filename in ('html/itch.html', 'static/js/load.js', 'static/png/atlas.png'):
            destination = self.root / filename
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(filename)
        stale = self.root / 'static/js/pkg/old.wasm'
        stale.parent.mkdir()
        stale.write_text('stale full game')
        (self.root / 'static/png/source.aseprite').write_text('source artwork')
        subprocess.run(['git', 'init', '-q', str(self.root)], check=True)
        subprocess.run(['git', '-C', str(self.root), '-c', 'user.name=Test',
                        '-c', 'user.email=test@example.invalid',
                        'commit', '-q', '--allow-empty', '-m', 'fixture'], check=True)
        bin_dir = self.root / 'bin'
        bin_dir.mkdir()
        for name in ('npm', 'wasm-pack', 'steamcmd', 'lipo'):
            tool = bin_dir / name
            tool.write_text(FAKE_TOOL)
            tool.chmod(0o755)
        packager = self.root / 'platforms/steam/node_modules/.bin/electron-packager'
        packager.parent.mkdir(parents=True)
        packager.write_text(FAKE_TOOL)
        packager.chmod(0o755)
        self.log = self.root / 'calls.jsonl'
        self.build = self.root / 'build output'
        self.env = dict(os.environ, PATH=f'{bin_dir}{os.pathsep}{os.environ["PATH"]}',
                        CALL_LOG=str(self.log), STEAM_USER='test-builder',
                        STEAMCMD=str(bin_dir / 'steamcmd'), STEAM_BUILD_ROOT=str(self.build),
                        STEAM_HOME=str(self.root / 'builder home'))

    def run_deploy(self, *args, **environment):
        return subprocess.run(['bash', str(self.root / 'deploy-steam.sh'), *args],
                              cwd=self.temp.name, env=dict(self.env, **environment),
                              text=True, capture_output=True)

    def calls(self):
        return [json.loads(line) for line in self.log.read_text().splitlines()] if self.log.exists() else []

    def test_both_editions_and_depots_are_built_before_one_login(self):
        stale = self.build / 'content/demo/windows/stale-full-game.wasm'
        stale.parent.mkdir(parents=True)
        stale.touch()
        result = self.run_deploy()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)
        self.assertFalse(stale.exists())
        calls = self.calls()
        self.assertEqual([c['tool'] for c in calls],
                         ['npm', 'wasm-pack', 'electron-packager', 'electron-packager',
                          'wasm-pack', 'electron-packager', 'electron-packager', 'steamcmd'])
        self.assertEqual(calls[-1]['home'], str(self.root / 'builder home'))
        self.assertEqual(calls[-1]['args'], ['+@ShutdownOnFailedCommand', '1', '+login', 'test-builder',
                         '+run_app_build', str(self.build / 'scripts/app_build_main.vdf'),
                         '+run_app_build', str(self.build / 'scripts/app_build_demo.vdf'), '+quit'])
        for variant, name, features, appid in (('main', 'Maginet', 'deploy', 2441960),
                                             ('demo', 'Maginet Demo', 'deploy,demo', 2529900)):
            content = self.build / 'content' / variant
            for app in (content / 'windows/resources/app',
                        content / f'macos/{name}.app/Contents/Resources/app'):
                self.assertEqual((app / 'static/js/pkg/maginet_bg.wasm').read_text(), features)
                self.assertFalse((app / 'static/js/pkg/old.wasm').exists())
                self.assertFalse((app / 'static/png/source.aseprite').exists())
            vdf = (self.build / f'scripts/app_build_{variant}.vdf').read_text()
            for identifier in (appid, appid + 1, appid + 2):
                self.assertIn(f'"{identifier}"', vdf)
            other_appid = 2529900 if variant == 'main' else 2441960
            self.assertNotIn(str(other_appid), vdf)
            self.assertNotIn('SetLive', vdf)
            self.assertIn(str(content), vdf)

    def test_build_only_never_uses_steam(self):
        result = self.run_deploy('--build-only', STEAM_USER='', STEAMCMD='/missing/steamcmd')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn('steamcmd', [c['tool'] for c in self.calls()])

    def test_failed_demo_build_prevents_all_uploads(self):
        result = self.run_deploy(FAIL_DEMO_BUILD='1')
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('steamcmd', [c['tool'] for c in self.calls()])

    def test_zero_exit_from_partial_upload_is_failure(self):
        result = self.run_deploy(FAIL_DEMO_UPLOAD='1')
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Upload not confirmed for AppID 2529900', result.stderr)
        self.assertNotIn('Both apps uploaded.', result.stdout)

    def test_steam_exit_code_is_preserved_through_tee(self):
        result = self.run_deploy(FAIL_STEAM_EXIT='1')
        self.assertEqual(result.returncode, 7)
        self.assertNotIn('Both apps uploaded.', result.stdout)

    def test_missing_login_fails_before_building(self):
        self.assertNotEqual(self.run_deploy(STEAM_USER='').returncode, 0)
        self.assertEqual(self.calls(), [])

    def test_unknown_track_fails_before_building(self):
        self.assertNotEqual(self.run_deploy('--beta').returncode, 0)
        self.assertEqual(self.calls(), [])


if __name__ == '__main__':
    unittest.main()
