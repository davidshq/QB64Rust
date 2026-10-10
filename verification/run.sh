#!/bin/sh
# Compile each verification program with the old compiler, run it, and record the output.
# Usage (Git Bash, from anywhere): verification/run.sh [name ...]   e.g. run.sh v01_numeric
# Writes <name>.compile.txt (compiler output) and <name>.out.txt (program output + exit code).
# The test runner does the work (tools/legacy_tests/README.md, "Verification programs"): the rules for
# <name>.stdin, <name>.noprompt, the 60 s timeout and the exit codes are there. This script only starts it.
here=$(cd "$(dirname "$0")" && pwd)
exec python "$here/../tools/legacy_tests/run_legacy_tests.py" --suite verification "$@"
