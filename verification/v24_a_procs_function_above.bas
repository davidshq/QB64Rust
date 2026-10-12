$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a FUNCTION defined above a DEFDBL for its letter and called
' below it, and one defined below a DEFxxx that is replaced before the call. The value 100000 + 1/3 shows the
' type of the result.
FUNCTION fa (x)
    fa = 100000 + 1# / 3
END FUNCTION
DEFDBL F
DIM r AS DOUBLE
r = fa!(3): PRINT "fa!(3), FUNCTION fa above DEFDBL F:"; r
r = fb#(3): PRINT "fb#(3), FUNCTION fb below it:"; r
SYSTEM
FUNCTION fb (x)
    fb = 100000 + 1# / 3
END FUNCTION
