DECLARE FUNCTION FuncB(n AS LONG)
DECLARE FUNCTION FuncC(n AS LONG)
PRINT FuncA(6)
END

FUNCTION FuncA(n AS LONG) AS LONG
    IF n <= 0 THEN
        FuncA = 0
    ELSE
        FuncA = FuncB(n - 1)
    END IF
END FUNCTION

FUNCTION FuncB(n AS LONG) AS LONG
    IF n <= 0 THEN
        FuncB = 0
    ELSE
        FuncB = FuncC(n - 1)
    END IF
END FUNCTION

FUNCTION FuncC(n AS LONG) AS LONG
    IF n <= 0 THEN
        FuncC = 0
    ELSE
        FuncC = FuncA(n - 1)
    END IF
END FUNCTION
