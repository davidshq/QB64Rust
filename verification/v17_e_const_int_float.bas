$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): is an integer-valued float CONST typed _INTEGER64 or DOUBLE? Multiplying
' by 2^62 tells them apart: _INTEGER64 wraps (2 * 2^62 prints -9223372036854775808), DOUBLE prints 9.2...D+18.
CONST f2 = 4 / 2, e2 = 2.5E+10, h2 = 2.5 * 2, lit = 2, big = 1E+19 / 1, sm = 2.5
PRINT "f2 = 4 / 2:"; f2 * 4611686018427387904
PRINT "lit = 2:"; lit * 4611686018427387904
PRINT "h2 = 2.5 * 2:"; h2 * 4611686018427387904
PRINT "e2 = 2.5E+10:"; e2 * 4611686018427387904
PRINT "sm = 2.5:"; sm * 4611686018427387904
PRINT "big = 1E+19 / 1:"; big
SYSTEM
