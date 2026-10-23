$CONSOLE
' Verification (m4-qb64pe-builtins, task 2.1): _DEFAULTCOLOR and _BACKGROUNDCOLOR of the destination after the
' start, COLOR in SCREEN 0 (with blink), SCREEN 12, SCREEN 13 and a 32-bit screen; of _NEWIMAGE images of each
' depth (text, 256 colours, 32 bits) before and after COLOR on them; of pages by number (0, a page of SCREEN 7
' that is not shown, one that does not exist); and of handles of no image (never made, freed): error 258. The
' values are printed on the console.
ON ERROR GOTO handler
show "start", _DEFAULTCOLOR, _BACKGROUNDCOLOR
COLOR 14, 1
show "screen 0", _DEFAULTCOLOR, _BACKGROUNDCOLOR
COLOR 31, 7
show "blink", _DEFAULTCOLOR, _BACKGROUNDCOLOR
SCREEN 12
show "screen 12", _DEFAULTCOLOR, _BACKGROUNDCOLOR
COLOR 14, 1
show "color 14, 1", _DEFAULTCOLOR, _BACKGROUNDCOLOR
SCREEN 13
COLOR 200, 100
show "screen 13", _DEFAULTCOLOR, _BACKGROUNDCOLOR
SCREEN _NEWIMAGE(64, 48, 32)
show "32 bits", _DEFAULTCOLOR, _BACKGROUNDCOLOR
COLOR _RGB32(1, 2, 3), _RGBA32(4, 5, 6, 7)
show "color 32", _DEFAULTCOLOR, _BACKGROUNDCOLOR
show "dest 0", _DEFAULTCOLOR(0), _BACKGROUNDCOLOR(0)
t& = _NEWIMAGE(20, 10, 0)
h8& = _NEWIMAGE(8, 8, 256)
h32& = _NEWIMAGE(8, 8, 32)
show "text image", _DEFAULTCOLOR(t&), _BACKGROUNDCOLOR(t&)
show "256 image", _DEFAULTCOLOR(h8&), _BACKGROUNDCOLOR(h8&)
show "32 image", _DEFAULTCOLOR(h32&), _BACKGROUNDCOLOR(h32&)
_DEST h8&: COLOR 9, 3
_DEST h32&: COLOR &HFF102030~&, &H80405060~&
_DEST 0
show "256 after", _DEFAULTCOLOR(h8&), _BACKGROUNDCOLOR(h8&)
show "32 after", _DEFAULTCOLOR(h32&), _BACKGROUNDCOLOR(h32&)
show "screen after", _DEFAULTCOLOR, _BACKGROUNDCOLOR
d! = h32&
show "single handle", _DEFAULTCOLOR(d!), _BACKGROUNDCOLOR(d!)
SCREEN 7, , 1, 0
COLOR 5, 2
show "page 1 written", _DEFAULTCOLOR(1), _BACKGROUNDCOLOR(1)
show "page 0 shown", _DEFAULTCOLOR(0), _BACKGROUNDCOLOR(0)
show "page 7", _DEFAULTCOLOR(7), _BACKGROUNDCOLOR(7)
show "page 8", _DEFAULTCOLOR(8), _BACKGROUNDCOLOR(8)
show "no image", _DEFAULTCOLOR(-12345), _BACKGROUNDCOLOR(-12345)
_FREEIMAGE h8&
show "freed", _DEFAULTCOLOR(h8&), _BACKGROUNDCOLOR(h8&)
c~& = 77
c~& = _BACKGROUNDCOLOR(-12345)
show "after error", c~&, 0
SYSTEM

handler:
d& = _DEST: _DEST _CONSOLE
PRINT "error"; ERR
_DEST d&
RESUME NEXT

SUB show (what$, f AS _UNSIGNED LONG, b AS _UNSIGNED LONG)
    d& = _DEST
    _DEST _CONSOLE
    PRINT what$; f; b
    _DEST d&
END SUB
