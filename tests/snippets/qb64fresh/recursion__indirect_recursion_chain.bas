DECLARE FUNCTION ChainB(n AS LONG)
DECLARE FUNCTION ChainC(n AS LONG)
DECLARE FUNCTION ChainD(n AS LONG)
PRINT ChainA(12)
END

FUNCTION ChainA(n AS LONG) AS LONG
    IF n <= 0 THEN
        ChainA = 0
    ELSE
        ChainA = ChainB(n - 1)
    END IF
END FUNCTION

FUNCTION ChainB(n AS LONG) AS LONG
    IF n <= 0 THEN
        ChainB = 0
    ELSE
        ChainB = ChainC(n - 1)
    END IF
END FUNCTION

FUNCTION ChainC(n AS LONG) AS LONG
    IF n <= 0 THEN
        ChainC = 0
    ELSE
        ChainC = ChainD(n - 1)
    END IF
END FUNCTION

FUNCTION ChainD(n AS LONG) AS LONG
    IF n <= 0 THEN
        ChainD = 0
    ELSE
        ChainD = ChainA(n - 1)
    END IF
END FUNCTION
