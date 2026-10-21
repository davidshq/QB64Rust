' TEST: check-fail
' The forms of the last built-ins of the upstream report the old compiler rejects (m4-builtins-param-types design
' D1; verification/v49_x40-x50): a wrong count, a number for a string, _OS$ stored into, CALL of the clipboard's
' statement; _CAST of a type it does not take, of a string or a whole TYPE value, in a CONST and with a wrong count
' (v49_x01-x15); CLEAR in a SUB or FUNCTION, with a parenthesis, four arguments or a string (x30-x35). Each is an error of the program, none "not supported yet".
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt
$CONSOLE:ONLY
PRINT _FILES$("a", "b")
PRINT _FILES$(5)
PRINT _MD5$(5)
PRINT _MD5$
PRINT _CLIPBOARD$("a")
_CLIPBOARD$ = 5
_OS$ = "a"
PRINT _OS$(1)
CALL _CLIPBOARD$("abc")
x = _CAST(STRING, 1)
x = _CAST(STRING * 4, 1)
x = _CAST(_BIT, 1)
x = _CAST(_UNSIGNED _BIT, 1)
x = _CAST(pt, 1)
x = _CAST(foo, 1)
x = _CAST(LONG, "1")
x = _CAST(LONG, p)
x = _CAST(LONG)
x = _CAST(LONG, 1, 2)
x = _CAST(_MEM, 1)
x = _CAST(1, LONG)
x = _CAST
x = _CAST()
CONST c = _CAST(LONG, 2.7)
CLEAR (1)
CLEAR , , LEN("a")
CLEAR 1, 2, 3, 4
CLEAR "a"
SYSTEM
SUB s
    CLEAR
END SUB
FUNCTION f
    CLEAR
END FUNCTION
