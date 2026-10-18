$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): the colour functions of the evaluator in mode 11
CONST c1 = _RGB(255, 0, 0, 11)
PRINT "_RGB(255, 0, 0, 11):"; HEX$(c1)
CONST c2 = _RGB(100, 150, 200, 11)
PRINT "_RGB(100, 150, 200, 11):"; HEX$(c2)
CONST c3 = _RGB(2, 3, 4, 11)
PRINT "_RGB(2, 3, 4, 11):"; HEX$(c3)
CONST c4 = _RGB(255, 255, 255, 11)
PRINT "_RGB(255, 255, 255, 11):"; HEX$(c4)
CONST c5 = _RGB(0, 0, 170, 11)
PRINT "_RGB(0, 0, 170, 11):"; HEX$(c5)
CONST c6 = _RGB(250, 80, 80, 11)
PRINT "_RGB(250, 80, 80, 11):"; HEX$(c6)
CONST c7 = _RGBA(100, 150, 200, 128, 11)
PRINT "_RGBA(100, 150, 200, 128, 11):"; HEX$(c7)
CONST c8 = _RED(0, 11)
PRINT "_RED(0, 11):"; c8
CONST c9 = _GREEN(0, 11)
PRINT "_GREEN(0, 11):"; c9
CONST c10 = _BLUE(0, 11)
PRINT "_BLUE(0, 11):"; c10
CONST c11 = _ALPHA(0, 11)
PRINT "_ALPHA(0, 11):"; c11
CONST c12 = _RED(1, 11)
PRINT "_RED(1, 11):"; c12
CONST c13 = _GREEN(1, 11)
PRINT "_GREEN(1, 11):"; c13
CONST c14 = _BLUE(1, 11)
PRINT "_BLUE(1, 11):"; c14
CONST c15 = _ALPHA(1, 11)
PRINT "_ALPHA(1, 11):"; c15
SYSTEM
