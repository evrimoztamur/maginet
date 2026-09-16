#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [[ "${1:-}" == --help ]]; then
  echo 'Usage: ./prepare-play-alpha.sh [path/to/android-release.env]'
  echo 'Builds a signed IAP-enabled AAB and draft alpha release metadata. Does not upload.'
  exit 0
fi
CONFIG_FILE="${1:-$HOME/.config/maginet/android-release.env}"
if [[ -f "$CONFIG_FILE" ]]; then
  # This is a trusted, user-owned shell configuration, kept outside the repo.
  source "$CONFIG_FILE"
elif [[ $# -gt 0 ]]; then
  echo 'Release configuration file not found.' >&2; exit 1
fi
if [[ -n "${MAGINET_PLAY_PUBLIC_KEY_FILE:-}" ]]; then
  MAGINET_PLAY_PUBLIC_KEY="$(cat "$MAGINET_PLAY_PUBLIC_KEY_FILE")"
  export MAGINET_PLAY_PUBLIC_KEY
fi
export CONFIGURATION=Release
./deploy-android.sh
AAB=android/app/build/outputs/bundle/release/app-release.aab
VERIFY_LOG="$(mktemp -t maginet-aab-verify)"
trap 'rm -f "$VERIFY_LOG"' EXIT
if ! jarsigner -J-Duser.language=en -J-Duser.country=US -verify "$AAB" > "$VERIFY_LOG" 2>&1 || [[ "$(cat "$VERIFY_LOG")" != *'jar verified.'* ]]; then
  echo 'The release bundle signature could not be verified.' >&2; exit 1
fi
python3 - <<'PY'
from pathlib import Path
import hashlib, json, os, shutil
source = Path('android/app/build/outputs/bundle/release/app-release.aab')
version = os.environ.get('MAGINET_VERSION_CODE', '1')
name = os.environ.get('MAGINET_VERSION_NAME', '1.0')
output = Path('android/build/play-alpha')
output.mkdir(parents=True, exist_ok=True)
bundle = output / f'maginet-{version}.aab'
shutil.copyfile(source, bundle)
notes = Path('android/play/release-notes-en-US.txt').read_text().strip()
release = {'track':'alpha', 'releases':[{'name':f'{name} alpha ({version})',
    'versionCodes':[version], 'status':'draft', 'releaseNotes':[{'language':'en-US','text':notes}]}]}
(output/'track.json').write_text(json.dumps(release, indent=2)+'\n')
receipt = {'packageName':'zone.evrim.maginet','track':'alpha','versionCode':int(version),
    'bundle':bundle.name,'sha256':hashlib.sha256(bundle.read_bytes()).hexdigest(),
    'productId':'zone.evrim.maginet.all','uploaded':False}
(output/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
print(f'Prepared signed bundle: {bundle}')
print(f'Draft alpha release metadata: {output / "track.json"}')
print('No upload performed. Verify the registered upload certificate and version code in Play Console.')
PY
