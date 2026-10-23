$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a TYPE member named fs$5, with no FUNCTION of that name.
TYPE t
    fs$5 AS STRING
END TYPE
DIM r AS t
PRINT "ok"
SYSTEM
