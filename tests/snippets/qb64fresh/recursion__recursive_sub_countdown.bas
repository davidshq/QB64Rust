CALL CountdownSub(5)
END

SUB CountdownSub(n AS LONG)
    IF n > 0 THEN
        PRINT n
        CALL CountdownSub(n - 1)
    END IF
END SUB
