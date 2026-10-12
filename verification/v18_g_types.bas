$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): arrays of each numeric type: initial value and formatting.
DIM i(1) AS INTEGER, l(1) AS LONG, q(1) AS _INTEGER64
DIM s(1) AS SINGLE, d(1) AS DOUBLE, f(1) AS _FLOAT
PRINT i(1); l(1); q(1); s(1); d(1); f(1)
i(1) = 32767: l(1) = 2147483647: q(1) = 9223372036854775807
s(1) = 1 / 3: d(1) = 1 / 3: f(1) = 1 / 3
PRINT i(1); l(1); q(1)
PRINT s(1); d(1); f(1)
i(0) = 2.5: l(0) = 3.5: q(0) = -2.5
PRINT i(0); l(0); q(0)
i(1) = i(1) + 1
PRINT "i(1) + 1:"; i(1)
SYSTEM
