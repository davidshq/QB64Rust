$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): RUN of a file that does not exist, under a handler.
ON ERROR GOTO h
RUN "v57_nosuch_program.exe"
PRINT "after RUN"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
