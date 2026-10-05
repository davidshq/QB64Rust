CALL PrintBinary(10)
PRINT
END

SUB PrintBinary(n AS LONG)
    IF n > 0 THEN
        CALL PrintBinary(n \ 2)
        PRINT n MOD 2;
    END IF
END SUB
