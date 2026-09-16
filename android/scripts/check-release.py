#!/usr/bin/env python3
"""Local release preflight. Never prints key material or passwords."""
import base64
import os
from pathlib import Path
import subprocess
import sys


def check(env):
    errors = []
    required = ('MAGINET_KEYSTORE', 'MAGINET_STORE_PASSWORD', 'MAGINET_KEY_ALIAS',
                'MAGINET_KEY_PASSWORD', 'MAGINET_PLAY_PUBLIC_KEY')
    errors += [f'{name} is not configured' for name in required if not env.get(name)]
    keystore = env.get('MAGINET_KEYSTORE')
    if keystore and not Path(keystore).is_file():
        errors.append('MAGINET_KEYSTORE does not identify a file')
    if env.get('MAGINET_KEY_ALIAS', '').lower() == 'androiddebugkey':
        errors.append('The Android debug key cannot be used as the release upload key')
    try:
        version = int(env.get('MAGINET_VERSION_CODE', '1'))
        if not 1 <= version <= 2_100_000_000:
            raise ValueError()
    except ValueError:
        errors.append('MAGINET_VERSION_CODE must be between 1 and 2100000000')
    value = ''.join(env.get('MAGINET_PLAY_PUBLIC_KEY', '').split())
    if value:
        try:
            der = base64.b64decode(value, validate=True)
            result = subprocess.run(['openssl', 'rsa', '-pubin', '-inform', 'DER', '-noout'],
                                    input=der, capture_output=True, timeout=10)
            if result.returncode:
                raise ValueError()
        except (ValueError, OSError, subprocess.TimeoutExpired):
            errors.append('MAGINET_PLAY_PUBLIC_KEY must be the Base64 RSA public key from Play Console')
    return errors


def main():
    errors = check(os.environ)
    if not errors:
        try:
            result = subprocess.run([
                'keytool', '-list', '-keystore', os.environ['MAGINET_KEYSTORE'],
                '-storepass:env', 'MAGINET_STORE_PASSWORD', '-alias', os.environ['MAGINET_KEY_ALIAS']
            ], capture_output=True, timeout=15)
            if result.returncode:
                errors.append('The upload keystore could not be opened with the configured alias/password')
        except (OSError, subprocess.TimeoutExpired):
            errors.append('Could not run keytool; configure JDK 17')
    if errors:
        print('Release preparation needs:', file=sys.stderr)
        for error in errors:
            print('  - ' + error, file=sys.stderr)
        return 1
    print('Release inputs validated locally. Play app access, registered upload certificate, and unused version code still require Console verification.')
    return 0


if __name__ == '__main__':
    sys.exit(main())
