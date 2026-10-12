$CONSOLE:ONLY
' Verification (m2-core-builtins review): does the old compiler accept this? (case_after_else)
x = 5
SELECT CASE x
    CASE ELSE: PRINT "else"
    CASE 5: PRINT "five"
END SELECT
SYSTEM
