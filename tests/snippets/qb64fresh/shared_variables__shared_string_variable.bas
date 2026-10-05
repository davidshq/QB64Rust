DIM message AS STRING
message = "Hello"
CALL AppendWorld
PRINT message
END

SUB AppendWorld
    SHARED message
    message = message + " World"
END SUB
