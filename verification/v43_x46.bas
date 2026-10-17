$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): a FUNCTION called in a single-line IF's condition that is
' continued with an underscore, after an error handler has run.
ON ERROR GOTO h
ERROR 5
x = 3
IF x = _
f THEN PRINT "no"
PRINT "after"
SYSTEM

h:
PRINT "error"; ERR; "_ERRORLINE"; _ERRORLINE
RESUME NEXT

FUNCTION f
    f = 0
    PRINT "in f"
END FUNCTION
