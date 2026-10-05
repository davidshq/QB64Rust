PRINT TailFactorial(5, 1)
END

FUNCTION TailFactorial(n AS LONG, acc AS LONG) AS LONG
    IF n <= 1 THEN
        TailFactorial = acc
    ELSE
        TailFactorial = TailFactorial(n - 1, n * acc)
    END IF
END FUNCTION
