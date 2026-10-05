PRINT SafeDivide(10.0, 2.0)
PRINT SafeDivide(10.0, 0.0)
END

FUNCTION SafeDivide(a AS DOUBLE, b AS DOUBLE) AS DOUBLE
    IF b = 0 THEN
        SafeDivide = 0.0
        EXIT FUNCTION
    END IF
    SafeDivide = a / b
END FUNCTION
