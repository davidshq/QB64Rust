' TEST: ir
' The font statements of m3-images-fonts-mouse in the IR (design D1, each call measured from the old compiler's
' C++ and compared by tests\callsite; verification\v31_x01_accepted): one built-in operation each, its slots and
' words. _FONT with and without its image, _PRINTSTRING with STEP and its image, _PRINTMODE with each of its
' words, _UPRINTSTRING with each optional slot in turn, _MAPUNICODE with TO; after CALL the same operations.
' _UCHARPOS with an element passes the array's bytes from it. The Settings line names the font part, which
' _UPRINTSTRING and _UCHARPOS need (design D2)
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM img AS LONG, t AS STRING, p(10) AS LONG, z(3, 3) AS LONG
_FONT 16
_FONT l, img
_FONT s, d
_FREEFONT l
_FREEFONT s
_PRINTSTRING (1, 2), "a"
_PRINTSTRING STEP(d, s), t, img
_PRINTSTRING (i, q), t, f
_PRINTMODE _KEEPBACKGROUND
_PRINTMODE _FILLBACKGROUND, img
_PRINTMODE _ONLYBACKGROUND, s
_UPRINTSTRING (1, 2), "abc"
_UPRINTSTRING (l, l), t, 100
_UPRINTSTRING (l, l), t, , 8
_UPRINTSTRING (l, l), t, , , l
_UPRINTSTRING (l, l), t, , , , img
_UPRINTSTRING (s, d), t, q, f, u, i
_MAPUNICODE 9786 TO 1
_MAPUNICODE s TO d
CALL _FONT(16)
CALL _PRINTMODE(_ONLYBACKGROUND)
CALL _PRINTSTRING((1, 2), "a")
CALL _UPRINTSTRING((1, 2), "abc")
CALL _MAPUNICODE(9786 TO 1)
CALL _FREEFONT(l)
l = _UCHARPOS(t, p(i))
l = _UCHARPOS(t, z(1, 2), 8)
