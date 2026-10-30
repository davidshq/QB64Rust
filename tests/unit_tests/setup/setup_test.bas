' Spec compiler/unit-tests, "Module-level code runs before each test": each test starts from it in a process of its
' own.
DIM SHARED n AS LONG
n = 5

SUB TestA
    n = n + 1
    AssertEqualInt 6, n, "after one step"
END SUB

SUB TestB
    AssertEqualInt 5, n, "untouched"
END SUB
