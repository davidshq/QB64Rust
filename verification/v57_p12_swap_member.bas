$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): SWAP (r.n), s.n.
TYPE t
    n AS LONG
END TYPE
DIM r AS t, s AS t
r.n = 1: s.n = 2
SWAP (r.n), s.n
PRINT r.n; s.n
SYSTEM
