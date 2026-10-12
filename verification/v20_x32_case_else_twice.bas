$CONSOLE:ONLY
' Verification (m2-core-builtins review): does the old compiler accept this? (case_else_twice)
x = 5
SELECT CASE x
    CASE 1: PRINT "one"
    CASE ELSE: PRINT "else 1"
    CASE ELSE: PRINT "else 2"
END SELECT
SYSTEM
