SUB SetValue(n AS INTEGER)
    n = 99
END SUB

DIM x AS INTEGER
x = 5
CALL SetValue(x)
PRINT x
