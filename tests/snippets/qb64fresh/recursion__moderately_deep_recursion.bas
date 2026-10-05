PRINT DeepCount(100)
END

FUNCTION DeepCount(n AS LONG) AS LONG
    IF n <= 0 THEN
        DeepCount = 0
    ELSE
        DeepCount = 1 + DeepCount(n - 1)
    END IF
END FUNCTION
