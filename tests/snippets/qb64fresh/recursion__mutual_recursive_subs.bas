DECLARE SUB SubB(n AS LONG)
CALL SubA(3)
END

SUB SubA(n AS LONG)
    IF n > 0 THEN
        PRINT "A:"; n
        CALL SubB(n - 1)
    END IF
END SUB

SUB SubB(n AS LONG)
    IF n > 0 THEN
        PRINT "B:"; n
        CALL SubA(n - 1)
    END IF
END SUB
