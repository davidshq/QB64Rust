' TEST: typed
$CONSOLE:ONLY
' The signs the old compiler drops before it reads an expression (known bug "a unary plus is a parse error";
' verification\v25_signs, v25_signs_const): a + right after a + or a - goes; then two - right after an operator
' go, so the last line joins two strings; - -x at the start of an expression stays two negations. In a CONST a
' run of signs is one sign, - when its number of - is odd
DIM x AS LONG, r AS DOUBLE, s AS STRING
r = -+x
r = 2 + +x
r = 2 - + +x
r = 2 ^ -+x * 3
r = 2 - + -x
r = 2 * - -x
r = 2 * - + - - -x
r = 2 - - -x
r = - -x
r = - - -x
r = NOT - + -x
s = "a" + +s + - -s
CONST c1 = -+5, c2 = 2 + +3, c3 = - - -3, c4 = +-3, c5 = 2 - -3, c6 = 2 + + - + -3
r = c1 + c2 + c3 + c4 + c5 + c6
SYSTEM
