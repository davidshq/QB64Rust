PRINT ProcessValue(100)
END

FUNCTION ProcessValue(n AS LONG) AS LONG
    DIM half AS LONG
    DIM quarter AS LONG
    DIM temp AS LONG
    IF n <= 1 THEN
        ProcessValue = n
    ELSE
        half = n \ 2
        quarter = n \ 4
        temp = half + quarter
        ProcessValue = temp + ProcessValue(n - temp)
    END IF
END FUNCTION
