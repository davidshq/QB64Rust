' TEST: check-fail
$CONSOLE:ONLY
' Forms of the image built-ins no test runs that the old compiler rejects, one a line (m3-images-fonts-mouse,
' design D1; verification\v31_x43, x44, x79-x84): each is an error here too, none a "not supported yet"
DIM l AS LONG
_DEPTHBUFFER
_DEPTHBUFFER 1
_CLIPBOARDIMAGE l
l = _CLIPBOARDIMAGE(1)
l = _SCREENIMAGE(1, 2)
l = _SCREENIMAGE()
_PRINTIMAGE
_PRINTIMAGE 1, 2
