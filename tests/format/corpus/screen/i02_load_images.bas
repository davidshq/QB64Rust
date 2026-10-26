' _LOADIMAGE of the files in i02_load_images.files (written by tools\screen\make_images.py): an 8-bit and a 24-bit
' BMP and a PNG with an alpha channel, in 32 bits, in 256 colours and in the screen's mode, and a file that is not
' there; each put on the screen, its size and some pixels printed (m3-images-fonts-mouse, task 2.4).
Screen _NewImage(240, 260, 32)
Cls , _RGB32(40, 40, 40)
a& = _LoadImage("pattern8.bmp", 32)
b& = _LoadImage("pattern24.bmp", 32)
c& = _LoadImage("alpha.png", 32)
d& = _LoadImage("pattern8.bmp", 256)
e& = _LoadImage("pattern24.bmp")
f$ = "alpha.png"
g& = _LoadImage(f$, , "")
m& = _LoadImage("missing.png", 32)
_PutImage (0, 0)-(31, 23), a&
_PutImage (40, 0)-(71, 23), b&
_PutImage (80, 0)-(111, 23), c&
_PutImage (120, 0)-(151, 23), d&
_PutImage (160, 0)-(191, 23), e&
_PutImage (0, 28), g&
Locate 5, 1
Print a& < -1; b& < -1; c& < -1; d& < -1; e& < -1; g& < -1; m&
Print _Width(a&); _Height(a&); _PixelSize(d&); _PixelSize(e&); _Width(g&)
_Source a&
Print Hex$(Point(0, 0)); " "; Hex$(Point(5, 3))
_Source b&
Print Hex$(Point(0, 0)); " "; Hex$(Point(15, 11))
_Source c&
Print Hex$(Point(0, 0)); " "; Hex$(Point(15, 11)); _Alpha32(Point(6, 0))
_Source d&
Print Point(3, 1); Hex$(_PaletteColor(5, d&))
_Source 0
System
