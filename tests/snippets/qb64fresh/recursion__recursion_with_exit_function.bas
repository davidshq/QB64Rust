PRINT FindFirst(1)
PRINT FindFirst(5)
PRINT FindFirst(10)
END

FUNCTION FindFirst(n AS LONG) AS LONG
    IF n > 7 THEN
        FindFirst = -1
        EXIT FUNCTION
    END IF
    IF n = 5 THEN
        FindFirst = 5
        EXIT FUNCTION
    END IF
    FindFirst = FindFirst(n + 1)
END FUNCTION
