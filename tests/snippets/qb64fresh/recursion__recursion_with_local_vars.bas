PRINT SumDigits(12345)
END

FUNCTION SumDigits(n AS LONG) AS LONG
    DIM digit AS LONG
    DIM rest AS LONG
    IF n = 0 THEN
        SumDigits = 0
    ELSE
        digit = n MOD 10
        rest = n \ 10
        SumDigits = digit + SumDigits(rest)
    END IF
END FUNCTION
