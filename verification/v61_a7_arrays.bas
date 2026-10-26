$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of the elements of a SUB's arrays: DIM, STATIC, a
' REDIM after a size is read (as ssimpire's viewspr of qbasic_testcases) and an array of a TYPE (its loadpal).
TYPE hues
    r AS _UNSIGNED _BYTE
    g AS _UNSIGNED _BYTE
    b AS _UNSIGNED _BYTE
END TYPE
s 3
s 5
SYSTEM

SUB s (n AS INTEGER)
    DIM w(1 TO 3) AS INTEGER
    STATIC v(1 TO 2) AS LONG
    DIM pal(255) AS hues
    w(1) = 5: v(2) = v(2) + 1
    DEF SEG = VARSEG(w(1))
    PRINT VARPTR(w(2)); PEEK(VARPTR(w(1))); VARPTR(v(2)); PEEK(VARPTR(v(2))); VARSEG(v(1))
    DEF SEG
    REDIM sprites(n) AS INTEGER
    sprites(n) = 300
    DEF SEG = VARSEG(sprites(0))
    PRINT VARPTR(sprites(0)); VARPTR(sprites(n)); PEEK(VARPTR(sprites(n)) + 1)
    POKE VARPTR(sprites(1)), 9
    DEF SEG
    PRINT sprites(1)
    pal(1).g = 40
    DEF SEG = VARSEG(pal(0))
    PRINT VARPTR(pal(0)); VARPTR(pal(1).g); PEEK(VARPTR(pal(0)) + 4)
    DEF SEG
END SUB
