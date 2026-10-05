$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): END SELECT missing.
x = 2
SELECT CASE x
    CASE 1
        PRINT "one"
    CASE ELSE
        PRINT "other"
PRINT "after"
SYSTEM
