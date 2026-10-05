DIM total AS DOUBLE
total = 100.0
PRINT AddToTotal(25.0)
PRINT total
END

FUNCTION AddToTotal(amount AS DOUBLE) AS DOUBLE
    SHARED total
    total = total + amount
    AddToTotal = total
END FUNCTION
