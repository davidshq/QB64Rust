$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): RUN of a CONST named with a suffix, written without it.
CONST p$ = "v57_nosuch_program.exe"
ON ERROR GOTO h
RUN p
PRINT "after RUN"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
