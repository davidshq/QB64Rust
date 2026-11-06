$CONSOLE:ONLY
' v68 "Empty SELECT CASE block": a SELECT CASE with a statement and no CASE before its END SELECT.
n = 1
SELECT CASE n
    PRINT "inside"
END SELECT
PRINT "after"
SYSTEM
