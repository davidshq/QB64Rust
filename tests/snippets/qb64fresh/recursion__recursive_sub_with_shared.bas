DIM total AS LONG
total = 0
CALL Accumulate(5)
PRINT total
END

SUB Accumulate(n AS LONG)
    SHARED total
    IF n > 0 THEN
        total = total + n
        CALL Accumulate(n - 1)
    END IF
END SUB
