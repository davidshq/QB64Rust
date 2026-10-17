$CONSOLE:ONLY
' Verification (m3-arraycopy, while its proposal was written): does the old compiler accept this? (76: a SUB
' whose _ARRAYCOPY takes its parameter as one dimension, called with an array of two)
DIM m(1 TO 2, 1 TO 2) AS LONG, b(1 TO 5) AS LONG
cp m(), b()
PRINT "ok"
SYSTEM

SUB cp (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(2)
END SUB
