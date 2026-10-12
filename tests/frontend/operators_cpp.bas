' TEST: cpp
$CONSOLE:ONLY
' Operators as emitted (m2-control-flow-slice task 3.2): one line per family
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE
' comparisons: -(a==b), floats narrowed; strings through the runtime
PRINT l = q; s < 2.1; d <> s; a$ >= b$; a$ = "x"
' bitwise: NOT, AND, OR, XOR, EQV, IMP; a float operand through qbr
PRINT NOT i; i AND l; l OR q; l XOR i; l EQV i; l IMP i; s AND 3
' short circuit and _NEGATE: &&, ||, !
PRINT l _ANDALSO i; l _ORELSE q; _NEGATE l
' integer division and MOD through the runtime's checks, power through pow2
PRINT l \ i; q MOD l; s ^ i
' variables on both sides of /: no `/*`
PRINT l / s; s / l
END
