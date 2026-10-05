PRINT Factorial(5)
PRINT Factorial(0)
PRINT Factorial(1)
END

FUNCTION Factorial(n AS LONG) AS LONG
    IF n <= 1 THEN
        Factorial = 1
    ELSE
        Factorial = n * Factorial(n - 1)
    END IF
END FUNCTION
