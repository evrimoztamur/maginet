#!/usr/bin/env python3
"""Create/reuse a private reviewer code and update the public native verifiers."""
import argparse
import hashlib
from pathlib import Path
import secrets
import re
import os

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--code-file', type=Path, default=Path.home() / '.config/maginet/reviewer-code.txt')
parser.add_argument('--rotate', action='store_true', help='replace the code; updated builds invalidate prior review access')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
args.code_file.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
if args.rotate or not args.code_file.exists():
    fd = os.open(args.code_file, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
    with os.fdopen(fd, 'w') as output:
        output.write('MAGINET-' + secrets.token_hex(12).upper() + '\n')
os.chmod(args.code_file, 0o600)
code = args.code_file.read_text().strip().upper()
if not re.fullmatch(r'MAGINET-[A-F0-9]{24}', code):
    raise SystemExit('Unexpected reviewer-code format; no verifier changed')
digest = hashlib.sha256(code.encode()).hexdigest()
for relative, pattern in [
    ('android/app/src/main/java/zone/evrim/maginet/ReviewerAccess.kt', r'(const val DIGEST = ")[^"]+(\")'),
    ('ios/Maginet/Store.swift', r'(static let codeDigest = ")[^"]+(\")'),
]:
    path = root / relative
    text, count = re.subn(pattern, lambda m: m[1] + digest + m[2], path.read_text())
    if count != 1:
        raise SystemExit('Expected exactly one verifier in ' + relative)
    path.write_text(text)
print('Native reviewer verifiers configured. Private code: ' + str(args.code_file))
