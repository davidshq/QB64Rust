PRINT TailSum(10, 0)
END

FUNCTION TailSum(n AS LONG, acc AS LONG) AS LONG
    IF n <= 0 THEN
        TailSum = acc
    ELSE
        TailSum = TailSum(n - 1, acc + n)
    END IF
END FUNCTION
