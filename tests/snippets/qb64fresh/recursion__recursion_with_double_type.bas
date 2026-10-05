PRINT Power(2.0, 8)
END

FUNCTION Power(b AS DOUBLE, exponent AS LONG) AS DOUBLE
    IF exponent = 0 THEN
        Power = 1.0
    ELSEIF exponent < 0 THEN
        Power = 1.0 / Power(b, -exponent)
    ELSE
        Power = b * Power(b, exponent - 1)
    END IF
END FUNCTION
