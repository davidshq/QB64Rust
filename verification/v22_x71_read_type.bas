$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (71_read_type)
TYPE rec
    n AS LONG
    m AS LONG
END TYPE
DIM r AS rec
DATA 1,2
READ r
PRINT r.n; r.m
