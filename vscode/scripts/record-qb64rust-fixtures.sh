#!/bin/sh
# Record the new compiler's output for the parser fixtures of a build with qb64rust (design D4 of extension-vsix).
# Usage (Git Bash, from anywhere): vscode/scripts/record-qb64rust-fixtures.sh
# Needs target/release/qb64rust.exe and the QB64pe reference clone next to the repo (its toolchain builds).
# For each case writes test-fixtures/qb64rust/<case>.out.txt (stdout+stderr, bytes as printed) and <case>.exit.
# The source is given by its absolute path, as the extension gives it; the fixture folder's absolute path is
# replaced by <FIXTURES> and the temporary output folder by <OUT>.
here=$(cd "$(dirname "$0")/.." && pwd)
q="$here/../target/release/qb64rust.exe"
fx="$here/test-fixtures/qb64rust"
if [ ! -x "$q" ]; then
    echo "qb64rust not found: $q" >&2
    exit 1
fi
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
tmpw=$(cygpath -w "$tmp")
fxw=$(cygpath -w "$fx")
# Backslashes doubled for use as a sed pattern.
fxe=$(echo "$fxw" | sed 's/\\/\\\\/g')
tmpe=$(echo "$tmpw" | sed 's/\\/\\\\/g')

# case name | source (relative to $fx)
cases="
build_type_error|type_error.bas
build_include_error|include_error/main.bas
build_marked|marked.bas
build_marked_only|marked_only.bas
build_warning|warning.bas
"

cd "$fx" || exit 1
echo "$cases" | while IFS='|' read -r name src; do
    [ -z "$name" ] && continue
    srcw="$fxw\\$(echo "$src" | sed 's|/|\\|g')"
    "$q" -x -w "$srcw" -o "$tmpw\\out.exe" > "$tmp/raw.txt" 2>&1
    echo $? > "$name.exit"
    sed -e "s/$fxe/<FIXTURES>/g" -e "s/$tmpe/<OUT>/g" "$tmp/raw.txt" > "$name.out.txt"
    echo "$name: exit $(cat "$name.exit")"
done
