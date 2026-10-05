DIM callCount AS LONG
callCount = 0
PRINT Fib(10)
PRINT "Calls:"; callCount
END

FUNCTION Fib(n AS LONG) AS LONG
    SHARED callCount
    callCount = callCount + 1
    IF n <= 1 THEN
        Fib = n
    ELSE
        Fib = Fib(n - 1) + Fib(n - 2)
    END IF
END FUNCTION
