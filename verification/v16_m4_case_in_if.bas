$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): CASE inside an IF inside a SELECT CASE.
SELECT CASE 1
    CASE 1
        IF 1 THEN
    CASE 2
        END IF
END SELECT
SYSTEM
