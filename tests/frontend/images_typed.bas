' TEST: typed
$CONSOLE:ONLY
' The colour and image functions of m3-images-fonts-mouse with their held and believed types (design D1;
' verification\v31_x01_accepted, v31_a_colours): _RGB and _RGBA an _UNSIGNED LONG of LONG slots with an optional
' handle; _RED to _ALPHA a LONG of _UNSIGNED LONG slots; the 32-bit parts a LONG; _HSB32 and _HSBA32 an
' _UNSIGNED LONG of DOUBLE slots; _HUE32, _SATURATION32, _BRIGHTNESS32 a DOUBLE; the image functions a LONG of
' LONG slots, a file name and a requirement strings; _LOADIMAGE with its middle slot left out; PMAP a SINGLE of a
' SINGLE and a LONG slot; _AUTODISPLAY no slot
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM t AS STRING
u = _RGB(1, 2, 3) + _RGB(i, l, q, s) + _RGB(d, f, u)
u = _RGBA(1, 2, 3, 4) + _RGBA(i, l, q, s, d)
l = _RED(u) + _GREEN(i, l) + _BLUE(d, f) + _ALPHA(u, u)
l = _RED32(u) + _GREEN32(i) + _BLUE32(s) + _ALPHA32(q)
u = _HSB32(120, 50, 50) + _HSB32(i, l, q) + _HSBA32(s, d, f, u)
d = _HUE32(u) + _SATURATION32(l) + _BRIGHTNESS32(s)
PRINT _RGB(1, 2, 3); HEX$(_RGBA(1, 2, 3, 4)); _RED(4) + 1; HEX$(_RED32(u)); _HSB32(1, 2, 3) + 1; _HUE32(i) / 2
l = _LOADIMAGE(t) + _LOADIMAGE("a.bmp", 32) + _LOADIMAGE(t, , "hardware") + _LOADIMAGE(t, s, t)
l = _COPYIMAGE(l) + _COPYIMAGE(d, q)
l = _WIDTH + _WIDTH(l) + _HEIGHT(s) + _PIXELSIZE(f) + _PIXELSIZE
l = _CLEARCOLOR + _CLEARCOLOR(u) + _BLEND + _BLEND(d)
l = _PALETTECOLOR(3) + _PALETTECOLOR(s, q)
l = _AUTODISPLAY
s = PMAP(50, 0) + PMAP(i, d) + PMAP(q, s)
PRINT HEX$(_LOADIMAGE(t)); _WIDTH + 1; HEX$(_PALETTECOLOR(1)); PMAP(1, 0) + 1; HEX$(PMAP(d, l)); _AUTODISPLAY
