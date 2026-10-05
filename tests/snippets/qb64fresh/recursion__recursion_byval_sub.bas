DIM x AS LONG
x = 5
CALL TestByVal(x)
PRINT x
END

SUB TestByVal(BYVAL n AS LONG)
    IF n > 0 THEN
        PRINT n
        CALL TestByVal(n - 1)
    END IF
END SUB
