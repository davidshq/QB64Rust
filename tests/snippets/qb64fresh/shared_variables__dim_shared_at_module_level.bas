DIM SHARED globalVar AS LONG
globalVar = 42
CALL PrintGlobal
END

SUB PrintGlobal
    SHARED globalVar
    PRINT globalVar
END SUB
