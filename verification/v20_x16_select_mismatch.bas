$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): does the old compiler accept this? (select_mismatch)
x = 1
SELECT CASE x
    CASE "a": PRINT "a"
END SELECT
