$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): a statement between SELECT CASE and the first CASE.
SELECT CASE 1
    PRINT "before case"
    CASE 1: PRINT "one"
END SELECT
SYSTEM
