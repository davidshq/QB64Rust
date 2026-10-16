$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): forms of the image, colour, font and mouse built-ins the
' old compiler accepts, as code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM img AS LONG, p(10) AS LONG, w(10) AS _UNSIGNED LONG, z(3, 3) AS LONG
_DEST _CONSOLE
PRINT "ok"
SYSTEM
' c01 rgb forms and types
PRINT _RGB(1, 2, 3)
PRINT _RGB(i, l, q, s)
PRINT _RGB(d, f, u)
PRINT HEX$(_RGB(1, 2, 3)); _RGB(1, 2, 3) + 1
PRINT _RGBA(1, 2, 3, 4)
PRINT _RGBA(i, l, q, s, d)
PRINT HEX$(_RGBA(1, 2, 3, 4, f)); _RGBA(1, 2, 3, 4) / 2
' c02 red green blue alpha
PRINT _RED(4); _GREEN(4); _BLUE(4); _ALPHA(4)
PRINT _RED(i, l); _GREEN(q, s); _BLUE(d, f); _ALPHA(u, u)
PRINT HEX$(_RED(4)); _RED(4) + 1; HEX$(_ALPHA(u))
' c03 32-bit parts
PRINT _RED32(&HFF102030); _GREEN32(i); _BLUE32(l); _ALPHA32(q)
PRINT _RED32(s); _GREEN32(d); _BLUE32(f); _ALPHA32(u)
PRINT HEX$(_RED32(u)); _RED32(u) + 1
' c04 hsb
PRINT _HSB32(120, 50, 50); _HSBA32(120, 50, 50, 50)
PRINT _HSB32(i, l, q); _HSBA32(s, d, f, u)
PRINT HEX$(_HSB32(1, 2, 3)); _HSB32(1, 2, 3) + 1
PRINT _HUE32(u); _SATURATION32(l); _BRIGHTNESS32(s)
PRINT _HUE32(i) + 1; _SATURATION32(d) / 2; _BRIGHTNESS32(f)
' c05 rgb32 now
PRINT _RGB32(1, 2, 3); _RGB32(5); _RGB32(5, 6); _RGB32(1, 2, 3, 4)
' i01 loadimage copyimage forms
l = _LOADIMAGE("a.bmp")
l = _LOADIMAGE("a.bmp", 32)
l = _LOADIMAGE("a.bmp", , "hardware")
l = _LOADIMAGE("a.bmp", 33, "hardware")
l = _LOADIMAGE("a.bmp", i)
l = _LOADIMAGE("a.bmp", s)
l = _LOADIMAGE("a.bmp", u)
PRINT HEX$(_LOADIMAGE("none.bmp", 32)); _LOADIMAGE("x") + 1
l = _COPYIMAGE(l)
l = _COPYIMAGE(l, 33)
l = _COPYIMAGE(d, q)
PRINT HEX$(_COPYIMAGE(0)); _COPYIMAGE(0) + 1
' i02 saveimage forms
_SAVEIMAGE "a.png"
_SAVEIMAGE "a.png", l
_SAVEIMAGE "a.png", , "bmp"
_SAVEIMAGE "a.png", l, "bmp"
_SAVEIMAGE "a.png", s
_SAVEIMAGE "a.png", u
' i03 size functions
PRINT _WIDTH; _HEIGHT; _PIXELSIZE
PRINT _WIDTH(l); _HEIGHT(i); _PIXELSIZE(q)
PRINT _WIDTH(s); _HEIGHT(d); _PIXELSIZE(f); _WIDTH(u)
PRINT HEX$(_WIDTH); _HEIGHT + 1; HEX$(_PIXELSIZE(0))
PRINT _WIDTH(_NEWIMAGE(64, 48, 32)); _RED32(&HFF102030)
' i04 clearcolor
_CLEARCOLOR 4
_CLEARCOLOR 4, l
_CLEARCOLOR _NONE
_CLEARCOLOR _NONE, l
_CLEARCOLOR u, s
_CLEARCOLOR d, q
_CLEARCOLOR i
PRINT _CLEARCOLOR; _CLEARCOLOR(l); HEX$(_CLEARCOLOR); _CLEARCOLOR(s) + 1
' i05 setalpha
_SETALPHA 128
_SETALPHA 128, , l
_SETALPHA 128, 5
_SETALPHA 128, 5 TO 9
_SETALPHA 128, 5 TO 9, l
_SETALPHA 128, 5, l
_SETALPHA s, d TO q, i
_SETALPHA u, u TO u, u
' i06 blend
_BLEND
_BLEND l
_BLEND s
_DONTBLEND
_DONTBLEND l
_DONTBLEND d
PRINT _BLEND; _BLEND(l); _BLEND(s); HEX$(_BLEND); _BLEND + 1
' i07 palettecolor copypalette
_PALETTECOLOR 3, _RGB32(10, 20, 30)
_PALETTECOLOR 3, u, l
_PALETTECOLOR s, d, q
_PALETTECOLOR i, l
PRINT _PALETTECOLOR(3); _PALETTECOLOR(3, l); _PALETTECOLOR(s, d); HEX$(_PALETTECOLOR(3)); _PALETTECOLOR(3) + 1
_COPYPALETTE
_COPYPALETTE l
_COPYPALETTE l, l
_COPYPALETTE , l
_COPYPALETTE s, d
' i08 autodisplay pmap
_AUTODISPLAY
PRINT _AUTODISPLAY; HEX$(_AUTODISPLAY); _AUTODISPLAY + 1
PRINT PMAP(50, 0); PMAP(i, l); PMAP(q, s); PMAP(d, f); PMAP(u, u)
PRINT HEX$(PMAP(1, 0)); PMAP(1, 0) + 1
' i09 putimage forms
_PUTIMAGE
_PUTIMAGE (10, 10), img
_PUTIMAGE (0, 0)-(99, 99), img, 0
_PUTIMAGE STEP(5, 5), img
_PUTIMAGE , img, 0, (0, 0)-(9, 9)
_PUTIMAGE (0, 0)-(9, 9), img, , , _SMOOTH
_PUTIMAGE (0, 0)-STEP(9, 9), img
_PUTIMAGE STEP(1, 1)-STEP(9, 9), img
_PUTIMAGE , img
_PUTIMAGE , , 0
_PUTIMAGE , img, 0
_PUTIMAGE , img, 0, (1, 1)
_PUTIMAGE , img, 0, STEP(1, 1)-STEP(3, 3)
_PUTIMAGE , img, 0, (1, 1)-(3, 3), _SMOOTH
_PUTIMAGE (1, 2), img, 0, (1, 1)-(3, 3), _SMOOTH
_PUTIMAGE , , , (1, 1)-(3, 3)
_PUTIMAGE , , , , _SMOOTH
_PUTIMAGE (x!, y#), h&, d%
_PUTIMAGE (i, l)-(q, s), d, f, (u, u)-(i, i)
_PUTIMAGE (1, 2)
_PUTIMAGE (1, 2)-(3, 4)
' i10 maptriangle 2d
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), 0
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), 0, _SMOOTH
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), , _SMOOTHSHRUNK
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50), , _SMOOTHSTRETCHED
_MAPTRIANGLE _SEAMLESS(0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE _CLOCKWISE (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE _ANTICLOCKWISE _SEAMLESS (0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9) TO (10, 10)-(50, 10)-(10, 50)
_MAPTRIANGLE (i, l)-(q, s)-(d, f), u TO (i, l)-(q, s)-(d, f), u
' i11 maptriangle 3d
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10, -1)-(50, 10, -1)-(10, 50, -1)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10, -1)-(50, 10, -1)-(10, 50, -1), 0, _SMOOTH
_MAPTRIANGLE _CLOCKWISE (0, 0)-(9, 0)-(0, 9), img TO (10, 10, -1)-(50, 10, -1)-(10, 50, -1)
_MAPTRIANGLE (0, 0)-(9, 0)-(0, 9), img TO (10, 10, -1)-(50, 10)-(10, 50)
' i12 depthbuffer
_DEPTHBUFFER ON
_DEPTHBUFFER OFF
_DEPTHBUFFER LOCK
_DEPTHBUFFER _CLEAR
_DEPTHBUFFER ON, l
_DEPTHBUFFER _CLEAR, s
' i13 screenimage clipboardimage printimage
l = _SCREENIMAGE
l = _SCREENIMAGE(1, 2, 3, 4)
l = _SCREENIMAGE(i, l, q, s)
PRINT HEX$(_SCREENIMAGE); _SCREENIMAGE + 1
l = _CLIPBOARDIMAGE
PRINT HEX$(_CLIPBOARDIMAGE); _CLIPBOARDIMAGE + 1
_CLIPBOARDIMAGE = l
_CLIPBOARDIMAGE = s
_PRINTIMAGE l
_PRINTIMAGE s
' i14 newimage freeimage
l = _NEWIMAGE(64, 48, 256)
l = _NEWIMAGE(64, 48)
_FREEIMAGE l
_FREEIMAGE
_FREEIMAGE s
' f01 loadfont
l = _LOADFONT("cour.ttf", 16)
l = _LOADFONT("cour.ttf", 16, "MONOSPACE")
l = _LOADFONT("cour.ttf", 16, , 1)
l = _LOADFONT("cour.ttf", 16, "BOLD", 1)
l = _LOADFONT("cour.ttf", i, "", q)
l = _LOADFONT("cour.ttf", s, "", d)
l = _LOADFONT("cour.ttf", u, "", f)
PRINT HEX$(_LOADFONT("cour.ttf", 16)); _LOADFONT("cour.ttf", 16) + 1
' f02 font statement and function
_FONT 16
_FONT 16, l
_FONT s, d
_FONT u, q
PRINT _FONT; _FONT(l); _FONT(s); HEX$(_FONT); _FONT + 1
_FREEFONT l
_FREEFONT s
' f03 font sizes
PRINT _FONTWIDTH; _FONTHEIGHT; _FONTWIDTH(l); _FONTHEIGHT(s)
PRINT HEX$(_FONTWIDTH); _FONTHEIGHT + 1
PRINT _PRINTWIDTH("abc"); _PRINTWIDTH("abc", l); _PRINTWIDTH("abc", s)
PRINT HEX$(_PRINTWIDTH("abc")); _PRINTWIDTH("abc") + 1
' f04 printstring
_PRINTSTRING (1, 2), "a"
_PRINTSTRING STEP(1, 2), "a", img
_PRINTSTRING (1, 2), "a", img
_PRINTSTRING (i, l), "a", q
_PRINTSTRING (s, d), "a", f
_PRINTSTRING (u, u), "a", u
' f05 printmode
_PRINTMODE _KEEPBACKGROUND
_PRINTMODE _FILLBACKGROUND, img
_PRINTMODE _ONLYBACKGROUND
_PRINTMODE _ONLYBACKGROUND, s
PRINT _PRINTMODE; _PRINTMODE(l); _PRINTMODE(s); HEX$(_PRINTMODE); _PRINTMODE + 1
' f06 unicode font functions
PRINT _UFONTHEIGHT; _UFONTHEIGHT(l); _UFONTHEIGHT(s); HEX$(_UFONTHEIGHT); _UFONTHEIGHT + 1
PRINT _ULINESPACING; _ULINESPACING(l); _ULINESPACING(s); HEX$(_ULINESPACING); _ULINESPACING + 1
PRINT _UPRINTWIDTH("abc"); _UPRINTWIDTH("abc", 8); _UPRINTWIDTH("abc", , l); _UPRINTWIDTH("abc", 8, l)
PRINT _UPRINTWIDTH("abc", s, d); HEX$(_UPRINTWIDTH("abc")); _UPRINTWIDTH("abc") + 1
' f07 uprintstring
_UPRINTSTRING (1, 2), "abc"
_UPRINTSTRING (1, 2), "abc", 100
_UPRINTSTRING (1, 2), "abc", , 8
_UPRINTSTRING (1, 2), "abc", , , l
_UPRINTSTRING (1, 2), "abc", , , , l
_UPRINTSTRING (1, 2), "abc", 100, 8, l, l
_UPRINTSTRING (s, d), "abc", q, f, u, i
' f08 ucharpos
n = _UCHARPOS("abc")
n = _UCHARPOS("abc", p())
n = _UCHARPOS("abc", p(), 8)
n = _UCHARPOS("abc", p(), 8, l)
n = _UCHARPOS("abc", p(), , l)
n = _UCHARPOS("abc", , 8)
n = _UCHARPOS("abc", p(2))
n = _UCHARPOS("abc", p(), s, d)
PRINT HEX$(_UCHARPOS("abc", p())); _UCHARPOS("abc") + 1
' f09 mapunicode
_MAPUNICODE 9786 TO 1
_MAPUNICODE l TO i
_MAPUNICODE s TO d
PRINT _MAPUNICODE(1); _MAPUNICODE(s); HEX$(_MAPUNICODE(1)); _MAPUNICODE(1) + 1
' m01 mouse functions
WHILE _MOUSEINPUT: WEND
PRINT _MOUSEINPUT; HEX$(_MOUSEINPUT); _MOUSEINPUT + 1
PRINT _MOUSEX; _MOUSEY; HEX$(_MOUSEX); _MOUSEY + 1
PRINT _MOUSEBUTTON(1); _MOUSEBUTTON(i); _MOUSEBUTTON(s); _MOUSEBUTTON(u); HEX$(_MOUSEBUTTON(1)); _MOUSEBUTTON(1) + 1
PRINT _MOUSEWHEEL; _MOUSEWHEEL(1); _MOUSEWHEEL(s); HEX$(_MOUSEWHEEL); _MOUSEWHEEL + 1
PRINT _MOUSEMOVEMENTX; _MOUSEMOVEMENTY; HEX$(_MOUSEMOVEMENTX); _MOUSEMOVEMENTY + 1
PRINT _MOUSEHIDDEN; _MOUSEDISABLED; HEX$(_MOUSEHIDDEN); _MOUSEDISABLED + 1
' m02 mousemove
_MOUSEMOVE 10, 20
_MOUSEMOVE i, l
_MOUSEMOVE q, s
_MOUSEMOVE d, f
_MOUSEMOVE u, u
' m03 call forms
CALL _MOUSEMOVE(10, 20)
CALL _PUTIMAGE((10, 10), img)
CALL _FONT(16)
CALL _PRINTMODE(_KEEPBACKGROUND)
CALL _SETALPHA(128, 5 TO 9)
CALL _CLEARCOLOR(_NONE)
CALL _BLEND
CALL _AUTODISPLAY
CALL _MAPUNICODE(9786 TO 1)
CALL _PRINTSTRING((1, 2), "a")
CALL _UPRINTSTRING((1, 2), "abc")
CALL _SAVEIMAGE("a.png")
CALL _COPYPALETTE
CALL _PALETTECOLOR(3, 4)
CALL _DONTBLEND(l)
CALL _FREEFONT(l)
CALL _MAPTRIANGLE((0, 0)-(9, 0)-(0, 9), img TO (10, 10)-(50, 10)-(10, 50))
CALL _DEPTHBUFFER(ON)
CALL _PRINTIMAGE(l)
' _UCHARPOS ulong array
n = _UCHARPOS("abc", w())
' _UCHARPOS 2d long array
n = _UCHARPOS("abc", z())
' _UCHARPOS undeclared array
n = _UCHARPOS("abc", nd&())
' _CLEARCOLOR none then color
_CLEARCOLOR _NONE 4
' _CLEARCOLOR bare
_CLEARCOLOR
