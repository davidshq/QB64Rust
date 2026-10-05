PRINT Ack(2, 2)
END

FUNCTION Ack(m AS LONG, n AS LONG) AS LONG
    IF m = 0 THEN
        Ack = n + 1
    ELSEIF n = 0 THEN
        Ack = Ack(m - 1, 1)
    ELSE
        Ack = Ack(m - 1, Ack(m, n - 1))
    END IF
END FUNCTION
