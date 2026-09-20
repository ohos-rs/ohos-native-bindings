#!/usr/bin/env bash
# Prepare a private full 2in1 image with Native child-process product policy.
set -euo pipefail

if [ "$#" -ne 2 ]; then
  echo "usage: $0 <full-2in1-package> <prepared-package>" >&2
  exit 2
fi
package_dir="$(cd "$1" && pwd)"
prepared_dir="$2"
test ! -e "$prepared_dir"
command -v debugfs >/dev/null
jq -e '.device_type == "2in1" and .capabilities.device_type_full == true' \
  "$package_dir/manifest.json" >/dev/null

# Keep the cached package, including its writable partitions, pristine.
case "$(uname -s)" in
  Darwin) cp -cR "$package_dir" "$prepared_dir" ;;
  Linux) cp -a --reflink=auto --sparse=always "$package_dir" "$prepared_dir" ;;
  *) echo "unsupported QEMU preparation host" >&2; exit 1 ;;
esac

image="$prepared_dir/images/system.img"
params="$prepared_dir/appfwk.para"
debugfs -R 'cat /etc/param/appfwk.para' "$image" >"$params" 2>/dev/null
python3 - "$params" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
text = path.read_text()
for name, value in [
    ("persist.sys.abilityms.multi_process_model", "true"),
    ("const.max_native_child_process", "50"),
]:
    pattern = rf"(?m)^{re.escape(name)}\s*=\s*[^\n]+$"
    text, count = re.subn(pattern, f"{name} = {value}", text)
    if count != 1:
        raise SystemExit(f"expected one definition of {name}, got {count}")
path.write_text(text)
PY
debugfs -w -R 'rm /etc/param/appfwk.para' "$image" >/dev/null 2>&1
debugfs -w -R "write \"$params\" /etc/param/appfwk.para" "$image" >/dev/null 2>&1
debugfs -R 'cat /etc/param/appfwk.para' "$image" >"$params" 2>/dev/null
grep -Fx 'persist.sys.abilityms.multi_process_model = true' "$params"
grep -Fx 'const.max_native_child_process = 50' "$params"

# The original checksums describe the released image, not this test copy.
rm "$prepared_dir/SHA256SUMS"
jq '.e2e_overrides = {
  "persist.sys.abilityms.multi_process_model": true,
  "const.max_native_child_process": 50
}' "$package_dir/manifest.json" >"$prepared_dir/manifest.json"
echo "Prepared full 2in1 QEMU package: $prepared_dir"
