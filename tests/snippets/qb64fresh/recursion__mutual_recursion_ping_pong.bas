DECLARE FUNCTION Pong(n AS LONG)
PRINT Ping(3)
END

FUNCTION Ping(n AS LONG) AS LONG
    IF n <= 0 THEN
        Ping = 0
    ELSE
        PRINT "Ping"; n
        Ping = Pong(n - 1)
    END IF
END FUNCTION

FUNCTION Pong(n AS LONG) AS LONG
    IF n <= 0 THEN
        Pong = 0
    ELSE
        PRINT "Pong"; n
        Pong = Ping(n - 1)
    END IF
END FUNCTION
