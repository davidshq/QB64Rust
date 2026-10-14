$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): the lock of a block of a variable is that of the procedure where _MEM stands,
' not of the one the variable lives in: a block of a main-module variable, and one of the SUB's STATIC
' variable, taken in a SUB stop being valid when the SUB returns.
DIM SHARED g AS LONG, gm AS _MEM, gs AS _MEM
DIM m AS _MEM
take
PRINT "after the SUB returned:"; _MEMEXISTS(gm); _MEMEXISTS(gs)
m = _MEM(g): PRINT "taken in the main module:"; _MEMEXISTS(m)
SYSTEM

SUB take
    STATIC st AS LONG
    gm = _MEM(g)
    gs = _MEM(st)
    PRINT "in the SUB:"; _MEMEXISTS(gm); _MEMEXISTS(gs)
END SUB
