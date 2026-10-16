' TEST: ir
' The image statements of m3-images-fonts-mouse that no test runs, in the IR (design D1 and D3, each call measured
' from the old compiler's C++ and compared by tests\callsite; verification\v31_x01_accepted): _CLIPBOARDIMAGE = h,
' _PRINTIMAGE h, _DEPTHBUFFER with each of its four words, with and without a handle, and the functions
' _SCREENIMAGE and _CLIPBOARDIMAGE; after CALL the same operations. The Settings line names the printer, which
' _PRINTIMAGE needs, and the screen-image part, which _SCREENIMAGE and _CLIPBOARDIMAGE need (design D2)
DIM i AS INTEGER, l AS LONG, s AS SINGLE
l = _SCREENIMAGE
l = _SCREENIMAGE(i, l, s, 4)
l = _CLIPBOARDIMAGE
_CLIPBOARDIMAGE = l
_CLIPBOARDIMAGE = s
_PRINTIMAGE l
_PRINTIMAGE s
_DEPTHBUFFER ON
_DEPTHBUFFER OFF
_DEPTHBUFFER LOCK
_DEPTHBUFFER _CLEAR
_DEPTHBUFFER ON, l
_DEPTHBUFFER _CLEAR, s
CALL _PRINTIMAGE(l)
CALL _DEPTHBUFFER(ON)
