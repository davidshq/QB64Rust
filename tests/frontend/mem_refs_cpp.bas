' TEST: cpp
$CONSOLE:ONLY
' The arguments of func__mem and the address of _OFFSET for the kinds of place the call-site check does not compare
' (an element's index and a member's offset are spelled apart; m3-mem task 3.1, design D3, verification\v32_a_blocks
' prints the values): an element of an array of numbers, of STRING * n and of a user type, from it to the array's
' end and alone; a member, a member of an element, an element of a member array of a variable and of an element, a
' whole member array; a whole array parameter, with the array's own lock (QB64PE-DIVERGENCES.md D-016).
TYPE t
    a AS INTEGER
    s AS STRING * 5
    v(0 TO 3) AS LONG
    d AS DOUBLE
END TYPE
DIM m AS _MEM, k AS LONG, u AS t, ut(3) AS t, b(10) AS INTEGER, fs(4) AS STRING * 3, b2(3, 4) AS LONG
m = _MEM(b(k))
m = _MEMELEMENT(b(4))
m = _MEM(fs(2))
m = _MEMELEMENT(fs(2))
m = _MEM(b2(1, 2))
m = _MEM(ut(1))
m = _MEMELEMENT(ut(1))
m = _MEM(u.d)
m = _MEMELEMENT(u.d)
m = _MEM(ut(1).d)
m = _MEM(u.v(2))
m = _MEMELEMENT(u.v(1))
m = _MEM(ut(1).v(2))
m = _MEM(ut(1).v())
PRINT _OFFSET(ut(1).v(2)); _OFFSET(fs(1)); _OFFSET(u.s)
y b()
SYSTEM

SUB y (q() AS INTEGER)
    DIM m AS _MEM
    m = _MEM(q())
    q(0) = 6
END SUB
