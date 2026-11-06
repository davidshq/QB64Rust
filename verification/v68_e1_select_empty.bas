$CONSOLE:ONLY
' v68 "Empty SELECT CASE block": a SELECT CASE followed directly by its END SELECT, and one with a CASE.
n = 1
SELECT CASE n
END SELECT
SELECT CASE n
    CASE 1
        PRINT "one"
END SELECT
SYSTEM
