$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN of a string variable naming a missing file, under a handler.
ON ERROR GOTO h
f$ = "v57_nosuch_program.exe"
RUN f$
PRINT "after RUN"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
