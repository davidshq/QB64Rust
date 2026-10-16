' TEST: check-fail
$CONSOLE:ONLY
' Forms of the mouse built-ins the old compiler rejects, one a line (m3-images-fonts-mouse, design D1;
' verification\v31_x13-x18, x101-x103, x107): each is an error here too, none a "not supported yet"
PRINT _MOUSEBUTTON
PRINT _MOUSEBUTTON(1, 2)
PRINT _MOUSEX(1)
PRINT _MOUSEWHEEL(1, 2)
_MOUSEMOVE 10
_MOUSEMOVE "a", 1
PRINT _MOUSEINPUT(1)
PRINT _MOUSEHIDDEN(1)
PRINT _MOUSEMOVEMENTX(1)
_MOUSEINPUT
