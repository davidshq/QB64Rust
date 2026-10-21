' TEST: typed
' The last built-ins of the upstream report (m4-builtins-param-types design D1): _OS$, _MD5$, _CLIPBOARD$ (function
' and statement) and _FILES$, bare and with its argument, each the plain call of its row; _CAST to each numeric
' type, a conversion typed as the type named (design D2); CLEAR bare, with arguments and after CALL (D4).
$CONSOLE:ONLY
DIM s AS STRING, f AS STRING * 4
s = _OS$
s = _MD5$(s + "x")
s = _md5$(f)
s = _CLIPBOARD$
_CLIPBOARD$ = s
_CLIPBOARD$ = f + "y"
s = _FILES$("*.txt")
s = _FILES$(f)
s = _FILES$
DIM sg AS SINGLE, d AS DOUBLE, k AS LONG, q AS _INTEGER64, u AS _UNSIGNED LONG, b AS _BYTE, fl AS _FLOAT
PRINT _CAST(_BYTE, sg); _CAST(_UNSIGNED _BYTE, k); _cast(integer, d); _CAST(_UNSIGNED INTEGER, q)
PRINT _CAST(LONG, d * 2); _CAST(_UNSIGNED LONG, b); _CAST(_INTEGER64, fl); _CAST(_UNSIGNED _INTEGER64, u)
PRINT _CAST(_OFFSET, sg); _CAST(_UNSIGNED _OFFSET, k); _CAST(SINGLE, q); _CAST(DOUBLE, k); _CAST(_FLOAT, 1)
k = _CAST(_BYTE, 300) + 1
CLEAR
CLEAR k + 1, , 2 * 3
CALL CLEAR
SYSTEM
