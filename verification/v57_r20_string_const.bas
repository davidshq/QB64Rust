$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): RUN of a string CONST without a suffix naming a missing file, under a handler.
CONST p = "v57_nosuch_program.exe"
ON ERROR GOTO h
RUN p
PRINT "after RUN"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
