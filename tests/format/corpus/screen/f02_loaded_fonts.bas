' Fonts loaded from Windows' font folders (m3-images-fonts-mouse, task 3.2, design D6; verification\v31_d_fonts):
' _LOADFONT of cour.ttf and arial.ttf by their bare names, with options, with a font index after a gap, and of a
' font that is not there; text drawn with each by PRINT, _PRINTSTRING and _UPRINTSTRING (each optional argument in
' turn, UTF-8 text, into an image); the sizes of a font and of a text, _UFONTHEIGHT, _ULINESPACING, _UPRINTWIDTH;
' _UCHARPOS into a whole LONG array, from an element, into an _UNSIGNED LONG array and one of two dimensions, and
' with no array; _FREEFONT, and the errors of a freed font and of freeing a built-in one.
Screen _NewImage(400, 320, 256)
Dim v(40) As Long, p(10) As Long, w(10) As _Unsigned Long, z(2, 3) As Long
Color 15, 1
Cls
f& = _LoadFont("cour.ttf", 16)
g& = _LoadFont("arial.ttf", 20)
m& = _LoadFont("arial.ttf", 20, "MONOSPACE")
k& = _LoadFont("cour.ttf", 12, , 0)
n& = _LoadFont("no-such-font.ttf", 16)
_Font f&
_PrintString (0, 0), "Courier New 16"
v(0) = _Font = f&: v(1) = _FontWidth: v(2) = _FontHeight: v(3) = _PrintWidth("abc")
v(4) = _UFontHeight: v(5) = _ULineSpacing: v(6) = _UPrintWidth("abc")
_Font g&
Locate 2, 1
Print "Arial 20 by PRINT"
v(7) = _FontWidth: v(8) = _FontHeight: v(9) = _PrintWidth("Wil")
v(10) = _UFontHeight(g&): v(11) = _ULineSpacing(g&): v(12) = _UPrintWidth("Wil", , g&)
_Font m&
_PrintString (0, 44), "Monospace Wil"
v(13) = _FontWidth(m&): v(14) = _UPrintWidth("Wil", 0, m&): v(15) = _PrintWidth("Wil")
_Font k&
_PrintString (0, 68), "Courier New 12"
v(16) = _FontHeight(k&)
' _UPRINTSTRING with the font of the destination, a width, an encoding, a font and an image.
_Font f&
_UPrintString (0, 84), "uprintstring"
_UPrintString (200, 84), "cut at 60 pixels", 60
t$ = "caf" + Chr$(195) + Chr$(169) + " UTF-8"
_UPrintString (0, 104), t$, , 8
_UPrintString (200, 104), "Arial", , , g&
img& = _NewImage(160, 30, 256)
_Dest img&
Cls , 4
_Dest 0
_UPrintString (2, 2), "in an image", , , k&, img&
_PutImage (200, 130), img&
_FreeImage img&
v(17) = _UPrintWidth(t$, 8): v(18) = _UPrintWidth(t$): v(19) = _UPrintWidth("cut at 60 pixels", 0, f&)
' _UCHARPOS: the starting pixel of each character, written from the element given.
c1 = _UCharPos("abc", p())
c2 = _UCharPos("Wil", w(), , g&)
c3 = _UCharPos("abcd", p(5), 0, f&)
c4 = _UCharPos("xy", z())
c5 = _UCharPos("abc")
c6 = _UCharPos(t$, z(1, 1), 8)
' Freeing: a freed font is an illegal function call to choose, and so is freeing a built-in one.
_Font 16
_FreeFont k&
On Error GoTo handler
e = 0: _Font k&: v(20) = e
e = 0: _FreeFont 16: v(21) = e
e = 0: v(22) = _FontWidth(k&): v(23) = e
On Error GoTo 0
_Font 8
Locate 22, 1
Print "handles:"; f& > 31; g& > 31; m& > 31; k& > 31; "not there:"; n&
Print "cour 16:"; v(0); v(1); v(2); v(3); v(4); v(5); v(6)
Print "arial 20:"; v(7); v(8); v(9); v(10); v(11); v(12)
Print "mono:"; v(13); v(14); v(15); "cour 12:"; v(16)
Print "uprintwidth:"; v(17); v(18); v(19)
Print "p:"; c1; p(0); p(1); p(2); p(3); p(4); p(5); p(6); p(7); p(8); p(9)
Print "w:"; c2; w(0); w(1); w(2); w(3); "none:"; c5
Print "z:"; c4; c6; z(0, 0); z(0, 1); z(0, 2); z(1, 1); z(1, 2); z(1, 3); z(2, 0); z(2, 1)
Print "freed:"; v(20); v(21); v(22); v(23)
System
handler:
e = Err
Resume Next
