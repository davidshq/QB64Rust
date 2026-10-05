PRINT Fibonacci(10)
END

FUNCTION Fibonacci(n AS LONG) AS LONG
    IF n <= 1 THEN
        Fibonacci = n
    ELSE
        Fibonacci = Fibonacci(n - 1) + Fibonacci(n - 2)
    END IF
END FUNCTION
