' Fonts loaded from Windows' font folders (m3-images-fonts-mouse, task 3.2, design D6; verification\v31_d_fonts):
' _LOADFONT of cour.ttf and arial.ttf by their bare names, with options, with a font index after a gap, and of a
' font that is not there; text drawn with each by PRINT, _PRINTSTRING and _UPRINTSTRING (each optional argument in
' turn, UTF-8 text, into an image); the sizes of a font and of a text, _UFONTHEIGHT, _ULINESPACING, _UPRINTWIDTH;
' _UCHARPOS into a whole LONG array, from an element, into an _UNSIGNED LONG array and one of two dimensions, and
' with no array; _FREEFONT, and the errors of a freed font and of freeing a built-in one.
SCREEN _NEWIMAGE(400, 320, 256)
DIM v(40) AS LONG, p(10) AS LONG, w(10) AS _UNSIGNED LONG, z(2, 3) AS LONG
COLOR 15, 1
CLS
f& = _LOADFONT("cour.ttf", 16)
g& = _LOADFONT("arial.ttf", 20)
m& = _LOADFONT("arial.ttf", 20, "MONOSPACE")
k& = _LOADFONT("cour.ttf", 12, , 0)
n& = _LOADFONT("no-such-font.ttf", 16)
_FONT f&
_PRINTSTRING (0, 0), "Courier New 16"
v(0) = _FONT = f&: v(1) = _FONTWIDTH: v(2) = _FONTHEIGHT: v(3) = _PRINTWIDTH("abc")
v(4) = _UFONTHEIGHT: v(5) = _ULINESPACING: v(6) = _UPRINTWIDTH("abc")
_FONT g&
LOCATE 2, 1
PRINT "Arial 20 by PRINT"
v(7) = _FONTWIDTH: v(8) = _FONTHEIGHT: v(9) = _PRINTWIDTH("Wil")
v(10) = _UFONTHEIGHT(g&): v(11) = _ULINESPACING(g&): v(12) = _UPRINTWIDTH("Wil", , g&)
_FONT m&
_PRINTSTRING (0, 44), "Monospace Wil"
v(13) = _FONTWIDTH(m&): v(14) = _UPRINTWIDTH("Wil", 0, m&): v(15) = _PRINTWIDTH("Wil")
_FONT k&
_PRINTSTRING (0, 68), "Courier New 12"
v(16) = _FONTHEIGHT(k&)
' _UPRINTSTRING with the font of the destination, a width, an encoding, a font and an image.
_FONT f&
_UPRINTSTRING (0, 84), "uprintstring"
_UPRINTSTRING (200, 84), "cut at 60 pixels", 60
t$ = "caf" + CHR$(195) + CHR$(169) + " UTF-8"
_UPRINTSTRING (0, 104), t$, , 8
_UPRINTSTRING (200, 104), "Arial", , , g&
img& = _NEWIMAGE(160, 30, 256)
_DEST img&
CLS , 4
_DEST 0
_UPRINTSTRING (2, 2), "in an image", , , k&, img&
_PUTIMAGE (200, 130), img&
_FREEIMAGE img&
v(17) = _UPRINTWIDTH(t$, 8): v(18) = _UPRINTWIDTH(t$): v(19) = _UPRINTWIDTH("cut at 60 pixels", 0, f&)
' _UCHARPOS: the starting pixel of each character, written from the element given.
c1 = _UCHARPOS("abc", p())
c2 = _UCHARPOS("Wil", w(), , g&)
c3 = _UCHARPOS("abcd", p(5), 0, f&)
c4 = _UCHARPOS("xy", z())
c5 = _UCHARPOS("abc")
c6 = _UCHARPOS(t$, z(1, 1), 8)
' Freeing: a freed font is an illegal function call to choose, and so is freeing a built-in one.
_FONT 16
_FREEFONT k&
ON ERROR GOTO handler
e = 0: _FONT k&: v(20) = e
e = 0: _FREEFONT 16: v(21) = e
e = 0: v(22) = _FONTWIDTH(k&): v(23) = e
ON ERROR GOTO 0
_FONT 8
LOCATE 22, 1
PRINT "handles:"; f& > 31; g& > 31; m& > 31; k& > 31; "not there:"; n&
PRINT "cour 16:"; v(0); v(1); v(2); v(3); v(4); v(5); v(6)
PRINT "arial 20:"; v(7); v(8); v(9); v(10); v(11); v(12)
PRINT "mono:"; v(13); v(14); v(15); "cour 12:"; v(16)
PRINT "uprintwidth:"; v(17); v(18); v(19)
PRINT "p:"; c1; p(0); p(1); p(2); p(3); p(4); p(5); p(6); p(7); p(8); p(9)
PRINT "w:"; c2; w(0); w(1); w(2); w(3); "none:"; c5
PRINT "z:"; c4; c6; z(0, 0); z(0, 1); z(0, 2); z(1, 1); z(1, 2); z(1, 3); z(2, 0); z(2, 1)
PRINT "freed:"; v(20); v(21); v(22); v(23)
SYSTEM
handler:
e = ERR
RESUME NEXT
