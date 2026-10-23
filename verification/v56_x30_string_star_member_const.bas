$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a TYPE member STRING * n with n a CONST above the block.
CONST n = 2
TYPE r
    f AS STRING * n
END TYPE
DIM x AS r
PRINT LEN(x)
PRINT "ok"
SYSTEM
