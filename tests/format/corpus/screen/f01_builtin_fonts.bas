' The built-in fonts and text drawn in graphics (m3-images-fonts-mouse, task 3.2; verification\v31_d_fonts): _FONT
' 8, 14 and 16 on the screen and on an image, _FONT, _FONTWIDTH, _FONTHEIGHT and _PRINTWIDTH of each, by handle
' and of the destination; _PRINTSTRING at a point, after STEP, into an image and after CALL; the three print modes
' over a striped box, on the screen and on an image, and _PRINTMODE of each; _MAPUNICODE set and read.
Screen _NewImage(320, 240, 256)
Dim v(30) As Long
Color 14, 1
_Font 8
_PrintString (0, 0), "font 8"
v(0) = _Font: v(1) = _FontWidth: v(2) = _FontHeight: v(3) = _PrintWidth("font 8")
_Font 14
_PrintString (0, 10), "font 14"
v(4) = _Font: v(5) = _FontWidth: v(6) = _FontHeight: v(7) = _PrintWidth("font 14")
_Font 16
_PrintString (0, 26), "font 16"
v(8) = _Font: v(9) = _FontWidth(8): v(10) = _FontHeight(14): v(11) = _FontWidth(16)
PSet (100, 0), 2
_PrintString Step(4, 4), "step"
Call _PrintString((160, 0), "call")
' The print modes over stripes: background filled, kept, or the background alone.
For x = 0 To 300 Step 4
    Line (x, 50)-(x + 1, 109), 4, BF
Next
Color 15, 2
v(12) = _PrintMode
_PrintMode _KeepBackground
_PrintString (4, 54), "keep the background"
v(13) = _PrintMode
_PrintMode _OnlyBackground
_PrintString (4, 72), "only the background"
v(14) = _PrintMode
_PrintMode _FillBackground
_PrintString (4, 90), "fill the background"
v(15) = _PrintMode
' An image with a font and a print mode of its own; the screen's stay as they are.
img& = _NewImage(120, 30, 256)
_Dest img&
Cls , 6
_Dest 0
_Font 8, img&
_PrintMode _KeepBackground , img&
_PrintString (2, 2), "image 8", img&
_Font 14, img&
_PrintString (2, 12), "image 14", img&
v(16) = _Font(img&): v(17) = _PrintWidth("image", img&): v(18) = _PrintMode(img&)
v(19) = _Font: v(20) = _PrintMode: v(21) = _PrintWidth("image")
_PutImage (190, 0), img&
_FreeImage img&
' _MAPUNICODE: the code page's character 1 is a smiling face; mapped to another code and read again.
v(22) = _MapUnicode(1): v(23) = _MapUnicode(65): v(24) = _MapUnicode(219)
_MapUnicode 9787 To 1
c% = 66: u& = 9731
_MapUnicode u& To c%
v(25) = _MapUnicode(1): v(26) = _MapUnicode(c%)
Color 15, 0
_Font 8
Locate 15, 1
Print "8:"; v(0); v(1); v(2); v(3)
Print "14:"; v(4); v(5); v(6); v(7)
Print "16:"; v(8); v(9); v(10); v(11)
Print "modes:"; v(12); v(13); v(14); v(15)
Print "image:"; v(16); v(17); v(18); "screen:"; v(19); v(20); v(21)
Print "map:"; v(22); v(23); v(24); v(25); v(26)
Print Hex$(_FontWidth); _FontHeight + 1; Hex$(_PrintMode); _MapUnicode(65) / 2
System
