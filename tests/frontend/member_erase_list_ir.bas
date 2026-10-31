' TEST: ir
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' The targets of one ERASE or REDIM, when a member array of one is found through an index (which may raise),
' are parts of one statement: each is cleared in turn and an error is handled after the last
' (m4-memory-records design D9; verification\v24_x312). Without such a target each is a statement of its own
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    s(0 TO 1) AS STRING
END TYPE
' A _DYNAMIC member array of an array element is released the same way, its index evaluated once where the old
' compiler evaluates it twice (QB64PE-DIVERGENCES.md D-055; verification\v64_a1), and given its new bounds the
' same way, its index evaluated once as in the old compiler (verification\v65_a1).
TYPE dy
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM a(3) AS t1, b(5) AS INTEGER, i AS INTEGER, y(3) AS dy, t AS dy
ERASE a(i).s, b, a(2).v
REDIM a(2).v(0 TO 3), a(i).s(0 TO 1)
ERASE b, b
ERASE y(f(1)).d, t.d, b
REDIM y(f(1)).d(1 TO 5), t.d(1 TO 6)
SYSTEM

FUNCTION f (n AS INTEGER)
    f = n
END FUNCTION
