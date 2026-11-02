' Spec compiler/unit-tests, "How a test is judged", "The assertion library" and "Input does not wait".
SUB EndsTheProgram
    SYSTEM
END SUB

SUB TestAssert
    PRINT "before"
    DIM x AS LONG
    _ASSERT 1 = 2, "math"
END SUB

SUB TestPass
    AssertTrue 1 + 1 = 2, "two"
    AssertFalse 1 = 2, "one is not two"
    AssertEqualStr "ab", "a" + "b", "join"
    AssertEqualDouble 0.5#, 1# / 2#, "half"
    AssertNear 3.14159#, ATN(1#) * 4#, 0.0001#, "pi"
END SUB

SUB TestDiv
    DIM a(3) AS LONG
    a(9) = 1
END SUB

SUB TestEnd
    EndsTheProgram
END SUB

SUB TestFake
    PRINT "PASS"
    PRINT "ok"
    ERROR 5
END SUB

SUB TestAdd
    AssertEqualInt 4, 2 + 3, "sum"
END SUB

SUB TestFailTwice
    Fail "one"
    Fail "two"
END SUB

SUB TestSingle
    DIM x AS SINGLE
    x = 0.1
    AssertEqualSingle 0.1, x, "x"
END SUB

SUB TestInput
    DIM a AS STRING
    LINE INPUT a
    AssertEqualStr "", a, "end of input"
END SUB
