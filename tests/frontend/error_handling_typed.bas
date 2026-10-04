' TEST: typed
$CONSOLE:ONLY
' Error handling in the typed tree: labels where they stand, the handler and resume targets, ERROR's value
' stored as a LONG, ERR (LONG, design D5), ERL (DOUBLE), CHR$ with a LONG slot
DIM k AS LONG, d AS DOUBLE
ON ERROR GOTO handler
ERROR 5
ERROR 2.5
ERROR d
back: PRINT CHR$(k); CHR$(d); ERR; ERL
setter
ON ERROR GOTO 0
SYSTEM
handler:
PRINT ERR
RESUME NEXT
RESUME
RESUME 0
RESUME back
done:

SUB setter
    ON ERROR GOTO handler
    ERROR 7
END SUB
