$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): the built-in fonts, fonts loaded from Windows' font folders,
' the sizes of a font and of a text, _UCHARPOS, _PRINTMODE and _MAPUNICODE, in a program with a hidden window.
' Each value is taken while the screen is the destination and printed to the console.
DIM v(20) AS LONG, p(10) AS LONG, u(10) AS _UNSIGNED LONG
SCREEN 12
v(0) = _FONT: v(1) = _FONTWIDTH: v(2) = _FONTHEIGHT: v(3) = _PRINTWIDTH("abc")
_FONT 8
v(4) = _FONT: v(5) = _FONTWIDTH: v(6) = _FONTHEIGHT: v(7) = _PRINTWIDTH("abc")
_FONT 14
v(8) = _FONT: v(9) = _FONTWIDTH: v(10) = _FONTHEIGHT
_FONT 16
v(11) = _FONTWIDTH(8): v(12) = _FONTHEIGHT(14)
_DEST _CONSOLE
PRINT "built-in:"; v(0); v(1); v(2); v(3); "8:"; v(4); v(5); v(6); v(7); "14:"; v(8); v(9); v(10); "by handle:"; v(11); v(12)
_DEST 0
f& = _LOADFONT("cour.ttf", 16)
g& = _LOADFONT("arial.ttf", 20)
m& = _LOADFONT("arial.ttf", 20, "MONOSPACE")
n& = _LOADFONT("no-such-font.ttf", 16)
_FONT f&
v(0) = _FONT: v(1) = _FONTWIDTH: v(2) = _FONTHEIGHT: v(3) = _PRINTWIDTH("abc")
v(4) = _UFONTHEIGHT: v(5) = _ULINESPACING: v(6) = _UPRINTWIDTH("abc")
v(7) = _FONTWIDTH(g&): v(8) = _FONTHEIGHT(g&): v(9) = _UFONTHEIGHT(g&): v(10) = _UPRINTWIDTH("Wil", , g&)
v(11) = _FONTWIDTH(m&): v(12) = _UPRINTWIDTH("Wil", 0, m&)
c = _UCHARPOS("abc", p())
c2 = _UCHARPOS("Wil", u(), , g&)
c3 = _UCHARPOS("abc")
_DEST _CONSOLE
PRINT "handles:"; f& > 31; g& > 31; m& > 31; "not there:"; n&
PRINT "cour 16:"; v(0) = f&; v(1); v(2); v(3); v(4); v(5); v(6)
PRINT "arial 20:"; v(7); v(8); v(9); v(10); "monospace:"; v(11); v(12)
PRINT "positions:"; c; p(0); p(1); p(2); p(3); p(4); "arial:"; c2; u(0); u(1); u(2); u(3); "no array:"; c3
PRINT "HEX$:"; HEX$(n&); " "; HEX$(_FONTWIDTH)
_DEST 0
_FONT 16
_FREEFONT f&
ON ERROR GOTO handler
e = 0: _FONT f&: fe = e
e = 0: _FREEFONT 16: fe2 = e
_DEST _CONSOLE: PRINT "freed font:"; fe; "free a built-in:"; fe2: _DEST 0
ON ERROR GOTO 0
v(0) = _PRINTMODE
_PRINTMODE _KEEPBACKGROUND
v(1) = _PRINTMODE
_PRINTMODE _ONLYBACKGROUND
v(2) = _PRINTMODE
_PRINTMODE _FILLBACKGROUND
v(3) = _PRINTMODE
v(4) = _MAPUNICODE(1)
_MAPUNICODE 9786 TO 1
v(5) = _MAPUNICODE(1)
v(6) = _MAPUNICODE(65)
_DEST _CONSOLE
PRINT "print mode:"; v(0); v(1); v(2); v(3)
PRINT "mapunicode:"; v(4); v(5); v(6)
SYSTEM
handler:
e = ERR
RESUME NEXT
