' Spec compiler/unit-tests, "A name of the library taken": the library's AssertTrue is defined twice.
SUB AssertTrue (c AS LONG, m AS STRING)
    PRINT m
END SUB

SUB TestMine
    AssertTrue 1, "mine"
END SUB
