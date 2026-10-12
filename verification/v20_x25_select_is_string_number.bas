$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): does the old compiler accept this? (select_is_string_number)
s$ = "a"
SELECT CASE s$
    CASE IS > 1: PRINT "1"
END SELECT
