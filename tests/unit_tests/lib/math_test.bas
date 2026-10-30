' Spec compiler/unit-tests, "Test files and test SUBs" and "Includes resolve from the test file".
'$INCLUDE:'math.bm'

SUB TestAdd
    AssertEqualInt 5, AddUp&(2, 3), "sum"
END SUB

SUB test_sub
    AssertEqualInt 1, AddUp&(3, -2), "difference"
END SUB

SUB TestWith (x AS LONG)
    PRINT x
END SUB

FUNCTION TestF
    TestF = 1
END FUNCTION

' SUB TestOld
SUB Helper
    PRINT "SUB TestStr"
END SUB
