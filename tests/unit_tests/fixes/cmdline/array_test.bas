' Spec compiler/unit-tests, "A fix on the command line": with whole-array-value on, PRINT a() is a compile error.
SUB TestArray
    DIM a(3) AS LONG
    PRINT a()
END SUB
