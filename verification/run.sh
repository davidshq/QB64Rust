#!/bin/sh
# Compile each verification program with the old compiler, run it, and record the output.
# Usage (Git Bash, from anywhere): verification/run.sh [name ...]   e.g. run.sh v01_numeric
# Writes <name>.compile.txt (compiler output) and <name>.out.txt (program output + exit code).
here=$(cd "$(dirname "$0")" && pwd)
qb="$here/../../QB64pe/qb64pe.exe"
cd "$here" || exit 1
# Without this, a program stopped by an untrapped runtime error opens a native message box that someone must
# click. With it, the runtime reports the error on stderr and ends the program (libqb error_handle.cpp).
export QB64PE_NOPROMPT=y
names="$*"
if [ -z "$names" ]; then
    names=$(ls v*.bas | sed 's/\.bas$//')
fi
for n in $names; do
    rm -f "$n.exe"
    "$qb" -x -q -m "$n.bas" -o "$here/$n.exe" > "$n.compile.txt" 2>&1
    echo "exit=$?" >> "$n.compile.txt"
    if [ -f "$n.exe" ]; then
        # <name>.noprompt (as in tests/corpus) overrides the setting, e.g. "continue" to go on after an error.
        if [ -f "$n.noprompt" ]; then
            QB64PE_NOPROMPT=$(tr -d '\r\n' < "$n.noprompt")
        else
            QB64PE_NOPROMPT=y
        fi
        # A hung program (e.g. screen PRINT with a comma under a redirected $CONSOLE) would block forever.
        timeout 60 ./"$n.exe" < /dev/null > "$n.out.txt" 2>&1
        rc=$?
        [ $rc -eq 124 ] && echo "(timed out after 60 s)" >> "$n.out.txt"
        echo "exit=$rc" >> "$n.out.txt"
    else
        echo "(not compiled)" > "$n.out.txt"
    fi
    echo "== $n"
    cat "$n.out.txt"
done
