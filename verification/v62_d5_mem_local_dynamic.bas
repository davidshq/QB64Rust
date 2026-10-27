$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 8, the block of a SUB's local dynamic array after the SUB
' returns: _MEMEXISTS, then a _MEMPUT through it (errors 300 to 315 are not trapped).
ON ERROR GOTO h
DIM SHARED gm AS _MEM
s
PRINT "1."; _MEMEXISTS(gm)
_MEMPUT gm, gm.OFFSET, 1 AS LONG
PRINT "2. after _MEMPUT"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    REDIM l(5) AS LONG
    gm = _MEM(l())
    PRINT "0."; _MEMEXISTS(gm)
END SUB
