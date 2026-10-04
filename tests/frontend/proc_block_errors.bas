' TEST: check-fail
$CONSOLE:ONLY
' Block errors of procedures (m2-procedures-and-errors D1): a stray END SUB, a nested SUB, the wrong END kind and
' a missing END FUNCTION. The main module and SUB a are still checked (the strings stored in numbers below); the
' body of a procedure whose header has the error (SUB b, FUNCTION c) is not
END SUB
x = "a"
SUB a
    y = "b"
SUB b
    z = "c"
END FUNCTION
w = "d"
FUNCTION c
    v = "e"
