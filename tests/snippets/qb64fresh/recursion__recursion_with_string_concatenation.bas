PRINT RepeatChar$("*", 5)
END

FUNCTION RepeatChar$(c AS STRING, n AS LONG) AS STRING
    IF n <= 0 THEN
        RepeatChar$ = ""
    ELSE
        RepeatChar$ = c + RepeatChar$(c, n - 1)
    END IF
END FUNCTION
