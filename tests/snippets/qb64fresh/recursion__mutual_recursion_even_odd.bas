DECLARE FUNCTION IsOdd(n AS LONG)
PRINT IsEven(4)
PRINT IsEven(5)
PRINT IsOdd(4)
PRINT IsOdd(5)
END

FUNCTION IsEven(n AS LONG) AS LONG
    IF n = 0 THEN
        IsEven = -1
    ELSE
        IsEven = IsOdd(n - 1)
    END IF
END FUNCTION

FUNCTION IsOdd(n AS LONG) AS LONG
    IF n = 0 THEN
        IsOdd = 0
    ELSE
        IsOdd = IsEven(n - 1)
    END IF
END FUNCTION
