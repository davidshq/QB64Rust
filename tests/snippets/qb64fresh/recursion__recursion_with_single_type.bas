PRINT Halve!(8.0)
END

FUNCTION Halve!(n AS SINGLE) AS SINGLE
    IF n < 1.0 THEN
        Halve! = n
    ELSE
        Halve! = Halve!(n / 2.0)
    END IF
END FUNCTION
