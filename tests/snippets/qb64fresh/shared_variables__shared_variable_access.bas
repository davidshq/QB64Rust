DIM counter AS LONG
counter = 10
CALL IncrementCounter
PRINT counter
END

SUB IncrementCounter
    SHARED counter
    counter = counter + 1
END SUB
