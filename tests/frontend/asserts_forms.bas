' TEST: check-ok
' The accepted forms of $ASSERTS, $ASSERTS:CONSOLE, $DEBUG and _ASSERT (m3-asserts-debug-zlib design D1, D3,
' task 2.1; verification/v33_x01-x03, x06, x13-x15, a11-a14, d08): any letter case, blanks before and after,
' twice, mixed with $CONSOLE:ONLY, inside a SUB, in a $IF branch that is not taken, after the last statement that
' runs; _ASSERT with and without its message, after CALL, in parentheses, with each numeric type.
   $asserts
$Asserts:Console
$CONSOLE:ONLY
   $debug
$DEBUG
$IF 0 THEN
    $ASSERTS x
$END IF
DIM k AS LONG, d AS DOUBLE, i64 AS _INTEGER64, s AS STRING, f AS STRING * 4, b AS _BYTE, u AS _UNSIGNED LONG
_ASSERT k
_ASSERT k, s
_ASSERT d, "m"
_ASSERT i64 = 1, f
_ASSERT b
_ASSERT u, s + "x"
_ASSERT (k)
CALL _ASSERT(k)
CALL _ASSERT(k, s)
x
SYSTEM
$ASSERTS:CONSOLE
$ASSERTS

SUB x
    $ASSERTS
    $DEBUG
    _ASSERT 1 = 1, "in a SUB"
END SUB
