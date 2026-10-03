#!/bin/bash
# Measures QB64Fresh (committed HEAD, scratch build) against QB64pe's test programs.
# Output: one TSV line per test: suite cat name kind parse check emit cc match
# Usage: QBF_WORK=<work dir> QBF_BIN=<dir with qb64fresh.exe> measure.sh
#   QBF_WORK  scratch directory for intermediate files and measure.tsv (required)
#   QBF_BIN   directory holding the built qb64fresh.exe (default: $QBF_WORK/qbf-target/debug)
#   QB64PE    QB64pe checkout with tests/ (default: ../../../QB64pe relative to this script)
# Needs MSYS2 UCRT64 gcc on PATH.
here=$(cd "$(dirname "$0")" && pwd)
S="${QBF_WORK:?set QBF_WORK to a scratch directory}"
QF="${QBF_BIN:-$S/qbf-target/debug}/qb64fresh.exe"
T="${QB64PE:-$here/../../../QB64pe}/tests"
W="$S/work"; rm -rf "$W"; mkdir -p "$W/res"
R="$S/measure.tsv"; : > "$R"
GCC=gcc
norm(){ tr -d '\r' < "$1" | sed -e ':a' -e '/^\n*$/{$d;N;ba' -e '}' ; }
i=0
find "$T/compile_tests" "$T/qbasic_testcases" -name '*.bas' | sort | while IFS= read -r f; do
  i=$((i+1)); d=$(dirname "$f"); cat=$(basename "$d"); n=$(basename "$f" .bas)
  suite=compile; case "$f" in *qbasic_testcases*) suite=qbasic;; esac
  kind=none; [ -f "$d/$n.output" ] && kind=output; [ -f "$d/$n.err" ] && kind=err
  ( cd "$d" && timeout 60 "$QF" "$n.bas" --ast > /dev/null 2> "$W/$i.parse.log" < /dev/null ); p=$?
  ( cd "$d" && timeout 60 "$QF" "$n.bas" --typed-ir > /dev/null 2> "$W/$i.check.log" < /dev/null ); c=$?
  emit=-; cc=-; match=-
  if [ "$kind" = output ] && [ $c -eq 0 ]; then
    ( cd "$d" && timeout 60 "$QF" "$n.bas" --emit-c --headless -o "$W/$i.c" > "$W/$i.emit.log" 2>&1 < /dev/null ); e=$?
    emit=$e
    if [ $e -eq 0 ] && [ -s "$W/$i.c" ]; then
      if timeout 180 "$GCC" -w -O0 -o "$W/$i.exe" "$W/$i.c" -lm -lgdi32 -luser32 -lwinmm -lws2_32 -lcomdlg32 -lole32 -lshell32 > "$W/$i.cc.log" 2>&1; then
        cc=OK
        ( cd "$d" && timeout 20 "$W/$i.exe" "$W/res" "$cat-$n" > "$W/$i.out" 2>&1 < /dev/null )
        if [ "$(norm "$W/$i.out" | md5sum)" = "$(norm "$d/$n.output" | md5sum)" ]; then match=YES; else match=NO; fi
        rm -f "$W/$i.exe"
      else cc=FAIL; fi
    fi
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$i" "$suite" "$cat" "$n" "$kind" "$p" "$c" "$emit" "$cc" "$match" >> "$R"
done
echo DONE
