' Spec compiler/unit-tests, "A timeout": TestLoop never returns; TestQuick still runs.
SUB TestLoop
    DO: LOOP
END SUB

SUB TestQuick
    AssertTrue 1, "quick"
END SUB
