' TEST: check-fail
$CONSOLE:ONLY
' The forms of _IIF, _MIN, _MAX and _CLAMP the old compiler rejects (m3-console-files tasks 2.2 and 2.4;
' verification\v30_x70-x81, x88-x90, x93, x94), one line each: each is an error that is not "not supported yet"
TYPE t
    n AS LONG
END TYPE
DIM l AS LONG, r AS t, r2 AS t, v AS STRING
PRINT _IIF(1, "a", 2)
PRINT _IIF(1, 2, "a")
PRINT _IIF("a", 1, 2)
PRINT _IIF(1, 2)
PRINT _IIF(1, 2, 3, 4)
r2 = _IIF(1, r, r2)
_IIF 1, 2, 3
PRINT _MIN("a", "b")
PRINT _MIN(1)
PRINT _MIN(1, 2, 3)
PRINT _CLAMP(1, 2)
PRINT _CLAMP("a", 1, 2)
l = _IIF 1, 2, 3
PRINT _MIN(r, 1)
PRINT _MAX(1, v)
PRINT _MAX
PRINT _IIF
l = _IIF()
CONST c1 = _IIF(1, 2, 3)
CONST c2 = _MIN(1, 2), c3 = _MAX(1, 2)
CONST c4 = _CLAMP(5, 1, 3)
