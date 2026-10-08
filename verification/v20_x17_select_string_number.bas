$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): does the old compiler accept this? (select_string_number)
s$ = "a"
SELECT CASE s$
    CASE 1: PRINT "1"
END SELECT
