#!/bin/sh
# Compile each verification program with the old compiler, run it, and record the output.
# Usage (Git Bash, from anywhere): verification/run.sh [name ...]   e.g. run.sh v01_numeric
# Writes <name>.compile.txt (compiler output) and <name>.out.txt (program output + exit code).
here=$(cd "$(dirname "$0")" && pwd)
qb="$here/../../QB64pe/qb64pe.exe"
cd "$here" || exit 1
names="$*"
# Programs that stop on an untrapped runtime error open a native message box that someone must click.
# They are skipped unless named explicitly.
popups="v02_errors v02b_idiv_zero v02c_no_handler"
if [ -z "$names" ]; then
    names=$(ls v*.bas | sed 's/\.bas$//')
    for p in $popups; do names=$(echo "$names" | grep -vx "$p"); done
fi
for n in $names; do
    rm -f "$n.exe"
    "$qb" -x -q -m "$n.bas" -o "$here/$n.exe" > "$n.compile.txt" 2>&1
    echo "exit=$?" >> "$n.compile.txt"
    if [ -f "$n.exe" ]; then
        ./"$n.exe" < /dev/null > "$n.out.txt" 2>&1
        echo "exit=$?" >> "$n.out.txt"
    else
        echo "(not compiled)" > "$n.out.txt"
    fi
    echo "== $n"
    cat "$n.out.txt"
done
