' TEST: check-fail
$CONSOLE:ONLY
' Fixed-length strings (m2-numeric-types tasks 7.1, 7.2): the old compiler's rejections as real errors
' (verification\v21_x31-x34: a length of 0, an expression, a float, a negative number; 2147483648 is negative in 32
' bits, which fails the old compiler's C++ build; `$0`), a string FOR variable; then what is not supported yet: a
' length named by a float constant, an array named with `$n`, `$n` beside a constant of its name (assigned, in
' `DIM`, in `SHARED`) or beside a `STRING * n` parameter of its name, a member whose length is a constant's name, a
' suffix on a fixed-length member, the `MID$` statement.
' The marked declarations stand last: they drop the real errors after them (the follow-on rule).
DIM z0 AS STRING * 0
DIM z1 AS STRING * (2 + 3)
DIM z2 AS STRING * 5.5
DIM z3 AS STRING * -1
DIM z4 AS STRING * 2147483648
DIM z5$0
DIM f AS STRING * 4
FOR f = 1 TO 2: NEXT
TYPE rec
    nm AS STRING * 6
END TYPE
DIM r AS rec
r.nm$ = "x"
CONST k = 2
k$3 = "x"
DIM k$4
MID$(f, 2, 1) = "x"
CONST half = 1.5
DIM h AS STRING * half
DIM a$3(2)
TYPE byconst
    s AS STRING * k
END TYPE
SYSTEM

SUB fp (x AS STRING * 5)
    SHARED k$2
    x$5 = "hi"
    PRINT x$5
END SUB
