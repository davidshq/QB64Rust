' Spec compiler/unit-tests, "A folder of its own": two tests write the same name into TestTempDir$ at once.
SUB WriteAndRead (text AS STRING)
    DIM f AS LONG, back AS STRING
    f = FREEFILE
    OPEN TestTempDir$ + "out.txt" FOR OUTPUT AS #f
    PRINT #f, text
    CLOSE #f
    _DELAY 0.5
    f = FREEFILE
    OPEN TestTempDir$ + "out.txt" FOR INPUT AS #f
    LINE INPUT #f, back
    CLOSE #f
    AssertEqualStr text, back, "read back"
END SUB

SUB TestOne
    WriteAndRead "one"
END SUB

SUB TestTwo
    WriteAndRead "two"
END SUB
