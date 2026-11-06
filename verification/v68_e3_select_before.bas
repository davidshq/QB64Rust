$CONSOLE:ONLY
' v68 "Empty SELECT CASE block": a statement before the first CASE of a SELECT CASE that has one.
n = 1
SELECT CASE n
    PRINT "before"
    CASE 1
        PRINT "one"
END SELECT
SYSTEM
