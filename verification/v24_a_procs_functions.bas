$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): the result of a FUNCTION typed by a default: defined below the
' statement, called without a suffix and with the matching one; its result assigned inside. The value 100000 +
' 1/3 shows the type: 100000 LONG, 100000.3359375 SINGLE, 100000.3333333333 DOUBLE.
DEFLNG F
DIM r AS DOUBLE
r = f1(3): PRINT "f1(3), FUNCTION f1 below DEFLNG F:"; r
r = f1&(3): PRINT "f1&(3):"; r
PRINT "the delta's scenario, f2(3) and f2&(3):"; f2(3); f2&(3)
r = g1(3): PRINT "g1(3), no default for G:"; r
r = g1!(3): PRINT "g1!(3):"; r
SYSTEM
FUNCTION f1 (x)
    f1 = 100000 + 1# / 3
END FUNCTION
FUNCTION f2 (x)
    f2 = x * 100000
END FUNCTION
FUNCTION g1 (x)
    g1 = 100000 + 1# / 3
END FUNCTION
