' TEST: check-fail
$CONSOLE:ONLY
' The old compiler's errors for statements this compiler marked (m4-statements-metacommands, design D9, D11): SHARED in
' the main module; a program FUNCTION as a statement, with CALL and without; an argument left out where the old
' compiler rejects it (INSTR's first stays marked: it compiles there); a CONST of the $IF branch only the old
' compiler's first pass compiles, used below (verification\v21_x56, v26_x34, v27_x50, v28_x49, v57_e13, v57_f01-f11,
' v57_o01-o21).
$CONSOLE:ONLY
$IF _CONSOLE_ = 2 THEN
    CONST k = 2
$ELSE
    CONST k = 0
$END IF
$CONSOLE
DIM x AS LONG
SHARED x
f 1
f(1)
CALL f(1)
CALL fs$5("a")
d = POINT(, 2)
d = POINT(1, )
u = _RGB32(1, , 3)
u = _RGB(1, , 3)
u$ = MID$("abc", 1, )
u$ = LEFT$(, 2)
u = INSTR(, "ab", "b")
u = f(1, )
PRINT k
SYSTEM
FUNCTION f (a)
    f = a
END FUNCTION
FUNCTION fs$5 (x AS STRING)
    fs$5 = x
END FUNCTION
