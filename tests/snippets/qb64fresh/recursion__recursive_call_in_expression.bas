DIM result AS LONG
result = Fact(5) + Fact(3) * Fact(2)
PRINT result
END

FUNCTION Fact(n AS LONG) AS LONG
    IF n <= 1 THEN
        Fact = 1
    ELSE
        Fact = n * Fact(n - 1)
    END IF
END FUNCTION
