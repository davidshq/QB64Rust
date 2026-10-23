' TEST: ir
$CONSOLE:ONLY
' The targets of one ERASE or REDIM, when a member array of one is found through an index (which may raise),
' are parts of one statement: each is cleared in turn and an error is handled after the last
' (m4-memory-records design D9; verification\v24_x312). Without such a target each is a statement of its own
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    s(0 TO 1) AS STRING
END TYPE
DIM a(3) AS t1, b(5) AS INTEGER, i AS INTEGER
ERASE a(i).s, b, a(2).v
REDIM a(2).v(0 TO 3), a(i).s(0 TO 1)
ERASE b, b
SYSTEM
