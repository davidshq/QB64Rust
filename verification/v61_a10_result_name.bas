$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR of the name of a FUNCTION without parameters inside it (with
' parameters the name is a call, "Incorrect number of arguments").
PRINT f%
SYSTEM

FUNCTION f%
    f% = 9
    PRINT VARPTR(f%)
END FUNCTION
