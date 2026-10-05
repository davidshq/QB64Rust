PRINT RecursiveWithLog(5)
END

SUB LogValue(msg AS STRING, val AS LONG)
    PRINT msg; val
END SUB

FUNCTION RecursiveWithLog(n AS LONG) AS LONG
    CALL LogValue("Processing: ", n)
    IF n <= 0 THEN
        RecursiveWithLog = 0
    ELSE
        RecursiveWithLog = n + RecursiveWithLog(n - 1)
    END IF
END FUNCTION
