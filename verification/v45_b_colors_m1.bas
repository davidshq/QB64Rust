$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): the colour functions of the evaluator in mode 1
CONST c1 = _RGB(255, 0, 0, 1)
PRINT "_RGB(255, 0, 0, 1):"; HEX$(c1)
CONST c2 = _RGB(100, 150, 200, 1)
PRINT "_RGB(100, 150, 200, 1):"; HEX$(c2)
CONST c3 = _RGB(2, 3, 4, 1)
PRINT "_RGB(2, 3, 4, 1):"; HEX$(c3)
CONST c4 = _RGB(255, 255, 255, 1)
PRINT "_RGB(255, 255, 255, 1):"; HEX$(c4)
CONST c5 = _RGB(0, 0, 170, 1)
PRINT "_RGB(0, 0, 170, 1):"; HEX$(c5)
CONST c6 = _RGB(250, 80, 80, 1)
PRINT "_RGB(250, 80, 80, 1):"; HEX$(c6)
CONST c7 = _RGBA(100, 150, 200, 128, 1)
PRINT "_RGBA(100, 150, 200, 128, 1):"; HEX$(c7)
CONST c8 = _RED(1, 1)
PRINT "_RED(1, 1):"; c8
CONST c9 = _GREEN(1, 1)
PRINT "_GREEN(1, 1):"; c9
CONST c10 = _BLUE(1, 1)
PRINT "_BLUE(1, 1):"; c10
CONST c11 = _ALPHA(1, 1)
PRINT "_ALPHA(1, 1):"; c11
CONST c12 = _RED(3, 1)
PRINT "_RED(3, 1):"; c12
CONST c13 = _GREEN(3, 1)
PRINT "_GREEN(3, 1):"; c13
CONST c14 = _BLUE(3, 1)
PRINT "_BLUE(3, 1):"; c14
CONST c15 = _ALPHA(3, 1)
PRINT "_ALPHA(3, 1):"; c15
SYSTEM
