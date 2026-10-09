' TEST: check-fail
$CONSOLE:ONLY
' The uses of the new types that m2-numeric-types task group 8 leaves out or rejects: a FUNCTION f$5 called as
' f$4 ("Name already in use", verification\v21_x49), `VAL` with `_UNSIGNED SINGLE` and a constant used with another
' suffix where one of them is `_BIT` (neither measured: "not supported yet"), and an array of `_BIT` (spec
' arrays-and-types). The `_BIT` array is a declaration marked "not supported yet", so it comes last (the follow-on rule).
' Also `x$n` beside a SUB x or a FUNCTION x& ("Name already in use", v21_x50, x53) and `CALL fs$5` (rejected,
' v21_x56: "not supported yet", not read as the FUNCTION's name).
CONST b`3 = 2, plain = 5
PRINT fs$4("a")
PRINT VAL("1", _UNSIGNED SINGLE)
PRINT b`3; b&
PRINT plain`3
s$5 = "abc"
PRINT fl$5
CALL fs$5("a")
DIM bits(3) AS _BIT * 3
SYSTEM

FUNCTION fs$5 (x AS STRING)
    fs$5 = x
END FUNCTION

SUB s
END SUB

FUNCTION fl&
    fl& = 1
END FUNCTION
