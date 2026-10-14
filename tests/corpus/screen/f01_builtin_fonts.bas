' The built-in fonts and text drawn in graphics (m3-images-fonts-mouse, task 3.2; verification\v31_d_fonts): _FONT
' 8, 14 and 16 on the screen and on an image, _FONT, _FONTWIDTH, _FONTHEIGHT and _PRINTWIDTH of each, by handle
' and of the destination; _PRINTSTRING at a point, after STEP, into an image and after CALL; the three print modes
' over a striped box, on the screen and on an image, and _PRINTMODE of each; _MAPUNICODE set and read.
SCREEN _NEWIMAGE(320, 240, 256)
DIM v(30) AS LONG
COLOR 14, 1
_FONT 8
_PRINTSTRING (0, 0), "font 8"
v(0) = _FONT: v(1) = _FONTWIDTH: v(2) = _FONTHEIGHT: v(3) = _PRINTWIDTH("font 8")
_FONT 14
_PRINTSTRING (0, 10), "font 14"
v(4) = _FONT: v(5) = _FONTWIDTH: v(6) = _FONTHEIGHT: v(7) = _PRINTWIDTH("font 14")
_FONT 16
_PRINTSTRING (0, 26), "font 16"
v(8) = _FONT: v(9) = _FONTWIDTH(8): v(10) = _FONTHEIGHT(14): v(11) = _FONTWIDTH(16)
PSET (100, 0), 2
_PRINTSTRING STEP(4, 4), "step"
CALL _PRINTSTRING((160, 0), "call")
' The print modes over stripes: background filled, kept, or the background alone.
FOR x = 0 TO 300 STEP 4
    LINE (x, 50)-(x + 1, 109), 4, BF
NEXT
COLOR 15, 2
v(12) = _PRINTMODE
_PRINTMODE _KEEPBACKGROUND
_PRINTSTRING (4, 54), "keep the background"
v(13) = _PRINTMODE
_PRINTMODE _ONLYBACKGROUND
_PRINTSTRING (4, 72), "only the background"
v(14) = _PRINTMODE
_PRINTMODE _FILLBACKGROUND
_PRINTSTRING (4, 90), "fill the background"
v(15) = _PRINTMODE
' An image with a font and a print mode of its own; the screen's stay as they are.
img& = _NEWIMAGE(120, 30, 256)
_DEST img&
CLS , 6
_DEST 0
_FONT 8, img&
_PRINTMODE _KEEPBACKGROUND, img&
_PRINTSTRING (2, 2), "image 8", img&
_FONT 14, img&
_PRINTSTRING (2, 12), "image 14", img&
v(16) = _FONT(img&): v(17) = _PRINTWIDTH("image", img&): v(18) = _PRINTMODE(img&)
v(19) = _FONT: v(20) = _PRINTMODE: v(21) = _PRINTWIDTH("image")
_PUTIMAGE (190, 0), img&
_FREEIMAGE img&
' _MAPUNICODE: the code page's character 1 is a smiling face; mapped to another code and read again.
v(22) = _MAPUNICODE(1): v(23) = _MAPUNICODE(65): v(24) = _MAPUNICODE(219)
_MAPUNICODE 9787 TO 1
c% = 66: u& = 9731
_MAPUNICODE u& TO c%
v(25) = _MAPUNICODE(1): v(26) = _MAPUNICODE(c%)
COLOR 15, 0
_FONT 8
LOCATE 15, 1
PRINT "8:"; v(0); v(1); v(2); v(3)
PRINT "14:"; v(4); v(5); v(6); v(7)
PRINT "16:"; v(8); v(9); v(10); v(11)
PRINT "modes:"; v(12); v(13); v(14); v(15)
PRINT "image:"; v(16); v(17); v(18); "screen:"; v(19); v(20); v(21)
PRINT "map:"; v(22); v(23); v(24); v(25); v(26)
PRINT HEX$(_FONTWIDTH); _FONTHEIGHT + 1; HEX$(_PRINTMODE); _MAPUNICODE(65) / 2
SYSTEM
