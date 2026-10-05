PRINT Countdown(5)
END

FUNCTION Countdown(n AS LONG) AS LONG
    IF n <= 0 THEN
        Countdown = 0
    ELSE
        PRINT n
        Countdown = Countdown(n - 1)
    END IF
END FUNCTION
