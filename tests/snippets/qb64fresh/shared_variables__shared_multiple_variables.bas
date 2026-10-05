DIM x AS LONG
DIM y AS LONG
x = 5
y = 10
CALL SwapValues
PRINT x; y
END

SUB SwapValues
    SHARED x, y
    DIM temp AS LONG
    temp = x
    x = y
    y = temp
END SUB
