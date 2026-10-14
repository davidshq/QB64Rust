' TEST: check-fail
$CONSOLE:ONLY
' Forms of the font built-ins the old compiler rejects, one a line (m3-images-fonts-mouse, design D1;
' verification\v31_x19-x26, x40-x42, x45-x48, x52-x71): each is an error here too, none a "not supported yet".
' _UCHARPOS takes an array of LONG or _UNSIGNED LONG only ("Expected LONG array-name")
DIM l AS LONG, n AS LONG, v AS LONG, q%(10), t$(10), p(10) AS LONG, y(10) AS _INTEGER64
n = _UCHARPOS("abc", q%())
n = _UCHARPOS("abc", v&)
n = _UCHARPOS("abc", t$())
n = _UCHARPOS("abc", p)
n = _UCHARPOS("abc", p(), 8, 0, 1)
n = _UCHARPOS
n = _UCHARPOS("abc", 5)
n = _UCHARPOS("abc", y())
_PRINTMODE
_PRINTMODE 1
PRINT _PRINTMODE(1, 2)
_MAPUNICODE 1, 2
_MAPUNICODE 1
PRINT _MAPUNICODE
PRINT _MAPUNICODE(1, 2)
_FONT
_FONT 1, 2, 3
PRINT _FONT(1, 2)
_FREEFONT
_FREEFONT 1, 2
_PRINTSTRING "a"
_PRINTSTRING (1, 2), 5
_PRINTSTRING (1, 2)
_PRINTSTRING (1, 2)-(3, 4), "a"
PRINT _PRINTWIDTH
PRINT _PRINTWIDTH(5)
_UPRINTSTRING (1, 2)
_UPRINTSTRING STEP(1, 2), "a"
_UPRINTSTRING (1, 2), "a", 1, 2, 3, 4, 5
PRINT _UPRINTWIDTH
PRINT _UFONTHEIGHT(1, 2)
l = _LOADFONT("cour.ttf")
l = _LOADFONT(5, 16)
l = _LOADFONT("cour.ttf", 16, "", 1, 2)
PRINT _FONTWIDTH(1, 2)
