$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): does the old compiler accept this? (select_to_mixed)
x = 1
SELECT CASE x
    CASE 1 TO "b": PRINT "a"
END SELECT
