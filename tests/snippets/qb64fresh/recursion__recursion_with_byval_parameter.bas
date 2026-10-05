PRINT DoubleRecursive(5)
END

FUNCTION DoubleRecursive(BYVAL n AS LONG) AS LONG
    IF n <= 0 THEN
        DoubleRecursive = 0
    ELSE
        DoubleRecursive = 2 + DoubleRecursive(n - 1)
    END IF
END FUNCTION
