PRINT SumTo(10)
END

FUNCTION SumTo(n AS LONG) AS LONG
    IF n <= 0 THEN
        SumTo = 0
    ELSE
        SumTo = n + SumTo(n - 1)
    END IF
END FUNCTION
