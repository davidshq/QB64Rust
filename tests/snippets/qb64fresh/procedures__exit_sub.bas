CALL Test(0)
CALL Test(5)
END

SUB Test(x AS LONG)
    IF x = 0 THEN EXIT SUB
    PRINT x
END SUB
