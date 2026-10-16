' TEST: typed
$CONSOLE:ONLY
' The font functions of m3-images-fonts-mouse with their held and believed types (design D1;
' verification\v31_x01_accepted, v31_d_fonts): each a LONG of LONG slots, a text a string; _LOADFONT and
' _UPRINTWIDTH with a middle slot left out; _UCHARPOS with its array as bytes (Bytes p), whole, from an element,
' of _UNSIGNED LONG and of two dimensions, or left out
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM t AS STRING, p(10) AS LONG, w(10) AS _UNSIGNED LONG, z(3, 3) AS LONG
l = _LOADFONT(t, 16) + _LOADFONT("cour.ttf", i, "MONOSPACE") + _LOADFONT(t, s, , 1) + _LOADFONT(t, u, t, f)
l = _FONT + _FONT(l) + _FONTWIDTH + _FONTWIDTH(s) + _FONTHEIGHT + _FONTHEIGHT(q)
l = _PRINTWIDTH(t) + _PRINTWIDTH("abc", d) + _PRINTMODE + _PRINTMODE(u)
l = _UFONTHEIGHT + _UFONTHEIGHT(l) + _ULINESPACING + _ULINESPACING(s)
l = _UPRINTWIDTH(t) + _UPRINTWIDTH(t, 8) + _UPRINTWIDTH(t, , l) + _UPRINTWIDTH(t, s, d)
l = _UCHARPOS(t) + _UCHARPOS(t, p()) + _UCHARPOS(t, p(), 8, l) + _UCHARPOS(t, p(), , l) + _UCHARPOS(t, , 8)
l = _UCHARPOS(t, p(2)) + _UCHARPOS(t, p(i)) + _UCHARPOS(t, w()) + _UCHARPOS(t, z()) + _UCHARPOS(t, z(1, 2))
l = _MAPUNICODE(1) + _MAPUNICODE(s)
PRINT HEX$(_LOADFONT(t, 16)); _FONT + 1; HEX$(_PRINTWIDTH(t)); _UFONTHEIGHT / 2; HEX$(_UCHARPOS(t, p()))
PRINT _MAPUNICODE(l) + 1; HEX$(_PRINTMODE); _UPRINTWIDTH(t) * 2
