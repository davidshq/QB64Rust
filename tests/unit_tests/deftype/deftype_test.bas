DEFSTR A-Z
OPTION _EXPLICIT
OPTION BASE 1
' Spec compiler/unit-tests, "Another default type": the library is typed explicitly, so these change nothing in it.
DIM SHARED greeting AS STRING
greeting = "hi"

SUB TestStrings
    AssertEqualStr "hi", greeting, "greeting"
END SUB

SUB TestCount
    DIM total AS LONG
    total = 2 + 3
    AssertEqualInt 4, total, "sum"
END SUB
