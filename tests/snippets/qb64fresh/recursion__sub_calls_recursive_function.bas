CALL PrintFactorials(5)
END

SUB PrintFactorials(maxN AS LONG)
    DIM i AS LONG
    FOR i = 0 TO maxN
        PRINT i; "! ="; Factorial(i)
    NEXT i
END SUB

FUNCTION Factorial(n AS LONG) AS LONG
    IF n <= 1 THEN
        Factorial = 1
    ELSE
        Factorial = n * Factorial(n - 1)
    END IF
END FUNCTION
