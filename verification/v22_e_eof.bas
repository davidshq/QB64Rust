$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Console input"): the input ends in the middle of the program
' (v22_e_eof.stdin holds one line): what the second INPUT does, what is printed, and the exit code.
ON ERROR GOTO h
DIM l AS LONG, s AS STRING
INPUT "first"; l
PRINT "<"; l; ">"
INPUT "second"; l
PRINT "<"; l; ">"
LINE INPUT "third "; s
PRINT "<"; s; ">"
PRINT "done"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
