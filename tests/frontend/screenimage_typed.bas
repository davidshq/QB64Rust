' TEST: typed
$CONSOLE:ONLY
' The image functions of m3-images-fonts-mouse that no test runs (design D1 and D3; verification\v31_x01_accepted):
' _SCREENIMAGE a LONG, bare or with four LONG slots, each number converted to LONG; _CLIPBOARDIMAGE a LONG with no
' slot; each in PRINT, HEX$ and an operation
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE
l = _SCREENIMAGE
l = _SCREENIMAGE(1, 2, 3, 4)
l = _SCREENIMAGE(i, l, q, s)
l = _SCREENIMAGE(d, 2.5, i, i)
PRINT HEX$(_SCREENIMAGE); _SCREENIMAGE + 1
l = _CLIPBOARDIMAGE
PRINT HEX$(_CLIPBOARDIMAGE); _CLIPBOARDIMAGE + 1
