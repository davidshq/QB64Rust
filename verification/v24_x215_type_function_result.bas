$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a FUNCTION declared with a user type as its result.
TYPE t1
    x AS LONG
END TYPE
DIM q AS t1
q = make
PRINT q.x
SYSTEM
FUNCTION make AS t1
    make.x = 5
END FUNCTION
