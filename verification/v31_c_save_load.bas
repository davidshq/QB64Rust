$CONSOLE
$SCREENHIDE
' Verification (m3-images-fonts-mouse, task 1.1): an image drawn, saved with _SAVEIMAGE in each format libqb
' writes, and loaded again with _LOADIMAGE; POINT of some pixels of each image loaded. The files are deleted.
DIM f$(5), v(5) AS _UNSIGNED LONG
f$(0) = "v31c.bmp": f$(1) = "v31c.png": f$(2) = "v31c.jpg": f$(3) = "v31c.tga": f$(4) = "v31c.qoi": f$(5) = "v31c.hdr"
SCREEN _NEWIMAGE(32, 32, 32)
CLS , _RGB32(10, 20, 30)
LINE (0, 0)-(15, 15), _RGB32(255, 0, 0), BF
PSET (20, 20), _RGB32(0, 255, 0)
src& = _COPYIMAGE(0)
p& = _NEWIMAGE(16, 16, 256)
_DEST p&
LINE (0, 0)-(7, 7), 4, BF
_DEST 0
ON ERROR GOTO handler
FOR i = 0 TO 5
    e = 0
    _SAVEIMAGE f$(i), src&
    se = e
    h& = _LOADIMAGE(f$(i), 32)
    IF h& < -1 THEN
        _SOURCE h&
        v(0) = POINT(1, 1): v(1) = POINT(20, 20): v(2) = POINT(30, 2)
        _SOURCE 0
        ww = _WIDTH(h&): hh = _HEIGHT(h&)
        _FREEIMAGE h&
    ELSE
        v(0) = 0: v(1) = 0: v(2) = 0: ww = 0: hh = 0
    END IF
    _DEST _CONSOLE
    PRINT f$(i); se; h& < -1; ww; hh; HEX$(v(0)); " "; HEX$(v(1)); " "; HEX$(v(2))
    _DEST 0
    IF _FILEEXISTS(f$(i)) THEN KILL f$(i)
NEXT
' A palette image saved with no format but the name's, loaded in 256 colours and in the screen's mode.
_SAVEIMAGE "v31c8", p&
h& = _LOADIMAGE("v31c8.png", 256)
h2& = _LOADIMAGE("v31c8.png")
_SOURCE h&
q = POINT(1, 1): q2 = POINT(10, 10)
_SOURCE 0
_DEST _CONSOLE
PRINT "no extension:"; _FILEEXISTS("v31c8.png"); _FILEEXISTS("v31c8.bmp"); h& < -1; _PIXELSIZE(h&); q; q2; _PIXELSIZE(h2&)
_DEST 0
IF _FILEEXISTS("v31c8.png") THEN KILL "v31c8.png"
IF _FILEEXISTS("v31c8.bmp") THEN KILL "v31c8.bmp"
' A format given in the third argument.
e = 0
_SAVEIMAGE "v31cx.dat", src&, "bmp"
_DEST _CONSOLE
PRINT "format given:"; e; _FILEEXISTS("v31cx.dat"); _FILEEXISTS("v31cx.dat.bmp")
IF _FILEEXISTS("v31cx.dat") THEN KILL "v31cx.dat"
IF _FILEEXISTS("v31cx.dat.bmp") THEN KILL "v31cx.dat.bmp"
e = 0
_SAVEIMAGE "v31cy.png", 12345
PRINT "bad handle:"; e; _FILEEXISTS("v31cy.png")
IF _FILEEXISTS("v31cy.png") THEN KILL "v31cy.png"
SYSTEM
handler:
e = ERR
RESUME NEXT
