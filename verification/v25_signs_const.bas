$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): in a CONST every run of + and - is one
' sign, - when the number of - in it is odd.
CONST c1 = -+5
CONST c2 = 2 + +3
CONST c3 = 2 - +3
CONST c4 = - - -3
CONST c5 = +-3
CONST c6 = 2 * +-3
CONST c7 = 2 - -3
CONST c8 = 2 - - -3
CONST c9 = 2 + -3
CONST c10 = 2 + + - + -3
CONST c11 = 2 ^ +-2
CONST c12 = 2 * -+3 + 1
CONST c13 = (-+5) * - - -2
CONST c14$ = "a" + +"b"
CONST c15 = -+-+-5
PRINT c1; c2; c3; c4; c5; c6; c7; c8
PRINT c9; c10; c11; c12; c13; c14$; c15
SYSTEM
