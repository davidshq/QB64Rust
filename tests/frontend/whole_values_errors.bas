' TEST: check-fail
$CONSOLE:ONLY
' A whole array where the old compiler rejects one (m4-line-numbers-whole-arrays, design D4; measured,
' `verification\v54_x10`, `x27`, `x29`, `x31`, `x32`, `x35`, `x37`, `x45`, `v36_x01`): the whole value of an
' assignment, also in parentheses; SWAP of two element types; a whole member array, also on both sides with type
' suffixes; a scalar parameter; LEN of an array of variable-length strings.
TYPE tm
    m(3) AS LONG
END TYPE
DIM a(2) AS LONG, i(2) AS INTEGER, s(2) AS STRING, t AS tm, u AS tm
t.m&() = u.m&()
x = a()
x = (a())
x$ = s()
SWAP a(), i()
PRINT t.m()
p a()
PRINT f(a())
PRINT LEN(s())
SYSTEM
SUB p (v AS LONG)
END SUB
FUNCTION f (v AS LONG)
END FUNCTION
