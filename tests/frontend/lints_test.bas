' TEST: warnings
' In a main file whose name ends in `_test.bas`, a test SUB is called by `qb64rust test`, not by the program: no
' `unused-procedure` warning for `TestAdd` or `testLower`. `TestWith` has a parameter, so it is no test.
$CONSOLE:ONLY
PRINT "tests"
SUB TestAdd
END SUB
SUB testLower
END SUB
SUB TestWith (x AS LONG)
    PRINT x
END SUB
