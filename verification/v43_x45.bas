$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): v43_d's last IF on one line: a FUNCTION f called in a
' single-line IF's condition after an error handler has run (v43_d stops there with "Division by zero").
ON ERROR GOTO h
ERROR 5
x = 3
IF x = f THEN PRINT "no"
PRINT "after"
SYSTEM

h:
PRINT "error"; ERR; "_ERRORLINE"; _ERRORLINE
RESUME NEXT

FUNCTION f
    f = 0
    PRINT "in f"
END FUNCTION
