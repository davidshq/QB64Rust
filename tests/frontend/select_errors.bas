' TEST: check-fail
$CONSOLE:ONLY
' SELECT CASE items of the wrong kind (verification\v20_x16, x17, x25, x26: "Expected numeric expression" /
' "Expected string expression" in the old compiler); the statements inside are still checked
x = 1: s$ = "a"
SELECT CASE x
    CASE "a": PRINT 1
END SELECT
SELECT CASE s$
    CASE 1: PRINT 2
END SELECT
SELECT CASE s$
    CASE IS > 1: PRINT 3
END SELECT
SELECT CASE x
    CASE 1 TO "b"
        y% = "inside"
END SELECT
' A CASE after CASE ELSE, and a second CASE ELSE ("Expected END SELECT", verification\v20_x31, x32)
SELECT CASE x
    CASE ELSE: PRINT 4
    CASE 5
        z% = "inside"
END SELECT
SELECT CASE x
    CASE 1: PRINT 5
    CASE ELSE: PRINT 6
    CASE ELSE: PRINT 7
END SELECT
