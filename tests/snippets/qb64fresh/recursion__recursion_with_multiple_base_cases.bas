PRINT Classify(15)
END

FUNCTION Classify(n AS LONG) AS LONG
    IF n <= 0 THEN
        Classify = 0
    ELSEIF n = 1 THEN
        Classify = 1
    ELSEIF n MOD 2 = 0 THEN
        Classify = Classify(n \ 2)
    ELSE
        Classify = Classify(3 * n + 1)
    END IF
END FUNCTION
