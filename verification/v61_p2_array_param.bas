$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of an element of an array parameter, called with a
' static array, a dynamic one, a COMMON SHARED one and an array of another SUB; a POKE reaches the caller's array.
COMMON SHARED c() AS INTEGER
DIM a(3) AS INTEGER
REDIM d(3) AS INTEGER
REDIM c(3) AS INTEGER
a(2) = 21: d(2) = 22: c(2) = 23
fill a()
fill d()
fill c()
PRINT a(2); d(2); c(2)
own
SYSTEM

SUB fill (z() AS INTEGER)
    DEF SEG = VARSEG(z(1))
    PRINT VARPTR(z(2)); PEEK(VARPTR(z(2)))
    POKE VARPTR(z(2)), PEEK(VARPTR(z(2))) + 100
    DEF SEG
END SUB

SUB own
    DIM o(3) AS INTEGER
    o(2) = 4
    fill o()
    PRINT o(2)
END SUB
