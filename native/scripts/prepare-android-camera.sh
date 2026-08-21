#!/usr/bin/env bash
set -euo pipefail

manifest="gen/android/app/src/main/AndroidManifest.xml"

if [ ! -f "$manifest" ]; then
  echo "Android manifest was not generated: $manifest" >&2
  exit 1
fi

if ! grep -q 'android.permission.CAMERA' "$manifest"; then
  sed -i '/<manifest/a\    <uses-permission android:name="android.permission.CAMERA" />\n    <uses-feature android:name="android.hardware.camera.any" android:required="false" />' "$manifest"
fi

grep -q 'android.permission.CAMERA' "$manifest"
echo "Android camera permission prepared."
