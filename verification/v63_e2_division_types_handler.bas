$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 12, \ and MOD by a zero of each integer type under a handler that
' resumes next, and the value the variable keeps. The old compiler ends at the first.
ON ERROR GOTO h
DIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64, ul AS _UNSIGNED LONG, r AS LONG
r = 5: b = 7: r = b \ (b - b): PRINT "byte:"; r
r = 5: i = 7: r = i MOD (i - i): PRINT "integer mod:"; r
r = 5: l = 7: r = l \ (l - l): PRINT "long:"; r
q = 7: PRINT "integer64:"; q \ (q - q)
ul = 7: PRINT "unsigned long mod:"; ul MOD (ul - ul)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
