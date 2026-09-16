import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('release', Path(__file__).parents[1] / 'check-release.py')
release = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release)


class ReleasePreflightTests(unittest.TestCase):
    def test_missing_inputs_fail_without_reporting_values(self):
        errors = release.check({})
        self.assertEqual(5, len(errors))
        self.assertTrue(all('not configured' in e for e in errors))

    def test_debug_key_and_invalid_version_are_rejected(self):
        errors = release.check({'MAGINET_KEY_ALIAS': 'androiddebugkey', 'MAGINET_VERSION_CODE': '0'})
        self.assertTrue(any('debug key' in e for e in errors))
        self.assertTrue(any('version' in e.lower() for e in errors))
        self.assertTrue(any('version' in e.lower() for e in release.check({'MAGINET_VERSION_CODE': '2147483647'})))

    def test_malformed_public_keys_are_rejected_without_echoing_them(self):
        for key in ['PRIVATE-invalid-key', 'bm90IGFuIFJTQSBrZXk=']:
            errors = release.check({'MAGINET_PLAY_PUBLIC_KEY': key})
            self.assertTrue(any('Base64 RSA' in e for e in errors))
            self.assertFalse(any(key in e for e in errors))

    def test_input_paths_and_public_key_whitespace(self):
        with tempfile.NamedTemporaryFile() as keystore:
            env = {'MAGINET_KEYSTORE': keystore.name, 'MAGINET_STORE_PASSWORD': 'secret',
                   'MAGINET_KEY_ALIAS': 'upload', 'MAGINET_KEY_PASSWORD': 'secret',
                   'MAGINET_PLAY_PUBLIC_KEY': ' Zm9v\n', 'MAGINET_VERSION_CODE': '12'}
            with patch.object(release.subprocess, 'run') as run:
                run.return_value.returncode = 0
                self.assertEqual([], release.check(env))
                self.assertEqual(b'foo', run.call_args.kwargs['input'])
            env['MAGINET_KEYSTORE'] = keystore.name + '-missing'
            with patch.object(release.subprocess, 'run') as run:
                run.return_value.returncode = 0
                self.assertTrue(any('does not identify a file' in e for e in release.check(env)))


if __name__ == '__main__':
    unittest.main()
