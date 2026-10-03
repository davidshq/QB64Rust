#!/bin/sh
# Record the old compiler's output for the parser fixtures (design D9).
# Usage (Git Bash, from anywhere): vscode/scripts/record-fixtures.sh
# For each case writes test-fixtures/compiler/<case>.out.txt (stdout+stderr, bytes as printed) and <case>.exit.
# A -y case that succeeds also writes <case>.formatted.bas. The fixture folder's absolute path is replaced by
# <FIXTURES> so the files do not depend on where the repo lives.
here=$(cd "$(dirname "$0")/.." && pwd)
qb="$here/../../QB64pe/qb64pe.exe"
fx="$here/test-fixtures/compiler"
if [ ! -x "$qb" ]; then
    echo "compiler not found: $qb" >&2
    exit 1
fi
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
tmpw=$(cygpath -w "$tmp")
# Backslashes doubled for use as a sed pattern. The temporary output folder becomes <OUT>.
fxw=$(cygpath -w "$fx" | sed 's/\\/\\\\/g')
tmpe=$(echo "$tmpw" | sed 's/\\/\\\\/g')

# case name | source (relative to $fx) | mode
cases="
check_clean|clean.bas|-z -q -w
check_error_main|error_main.bas|-z -q -w
check_error_indented|error_indented.bas|-z -q -w
check_include_error|include_error/main.bas|-z -q -w
check_missing_include|missing_include/main.bas|-z -q -w
check_warning_unused|warning_unused.bas|-z -q -w
check_error_progress|error_main.bas|-z -x -w
build_clean|clean.bas|-c -x -w
format_ok|format_ok.bas|-y -q
format_cp437|format_cp437.bas|-y -q
format_error|error_main.bas|-y -q
"

cd "$fx" || exit 1
echo "$cases" | while IFS='|' read -r name src mode; do
    [ -z "$name" ] && continue
    rm -f "$tmp/out.bas"
    case "$mode" in
        -y*) target="$tmpw\\out.bas" ;;
        *) target="$tmpw\\check.exe" ;;
    esac
    # shellcheck disable=SC2086
    "$qb" $mode "$src" -o "$target" > "$tmp/raw.txt" 2>&1
    echo $? > "$name.exit"
    sed -e "s/$fxw/<FIXTURES>/g" -e "s/$tmpe/<OUT>/g" "$tmp/raw.txt" > "$name.out.txt"
    if [ -f "$tmp/out.bas" ]; then
        cp "$tmp/out.bas" "$name.formatted.bas"
    fi
    echo "$name: exit $(cat "$name.exit")"
done
