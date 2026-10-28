$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 13, the block of a SUB's local after the SUB is left by
' RESUME NEXT (a main-module handler calls it; the handler then goes on with GOTO, since the error is no longer
' being handled), beside the same SUB left by its end.
ON ERROR GOTO h
DIM SHARED gm AS _MEM
DIM SHARED leave AS LONG
leave = 0
s
PRINT "1. after the end of s:"; _MEMEXISTS(gm)
leave = 1
ERROR 5
back:
PRINT "2. after RESUME NEXT in s:"; _MEMEXISTS(gm)
SYSTEM
h:
PRINT "handler"; ERR
s
PRINT "after s in the handler"
GOTO back

SUB s
    DIM x AS LONG
    gm = _MEM(x)
    IF leave THEN RESUME NEXT
END SUB
