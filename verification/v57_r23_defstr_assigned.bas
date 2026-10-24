$CONSOLE:ONLY
' Verification (review of m4-statements-metacommands): RUN of a variable without a suffix under DEFSTR, assigned after the RUN.
DEFSTR A-Z
ON ERROR GOTO h
RUN p
PRINT "after RUN"
SYSTEM
p = "v57_nosuch_program.exe"
h:
PRINT "error"; ERR
RESUME NEXT
