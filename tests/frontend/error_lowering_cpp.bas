' TEST: cpp
$CONSOLE:ONLY
' Lowering pairs for error handling (design D6): SetHandler, Raise, the three Resume forms, labels as positions
' (before a statement, on their own line, after the last statement), CHR$ raising inside a PRINT
DIM k AS LONG
ON ERROR GOTO handler
k = -1
back: PRINT "a"; CHR$(k); "b"
ERROR 2.5
f& = twice&(3)
ON ERROR GOTO h2
ON ERROR GOTO 0
SYSTEM
handler:
PRINT ERR; ERL
k = 65
RESUME
h2:
RESUME NEXT
RESUME back
last:

FUNCTION twice& (a AS LONG)
    ON ERROR GOTO h2
    ERROR 7
    twice& = a * 2
END FUNCTION
