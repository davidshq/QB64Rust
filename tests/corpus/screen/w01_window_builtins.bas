' Screen program (m4-coverage-review, design D5): the window built-ins that act on the program's own window or
' read a value of the machine, run. A value that differs between runs or machines (the window's place, the
' desktop's size, the focus) is printed as a property or not at all. So is _TITLE$: read back after _TITLE, the
' old compiler's build gave five bytes that differ from run to run, not the title (on the runner's hidden desktop).
SCREEN 12
_TITLE "w01 window"
t$ = _TITLE$
_MOUSEHIDE
_MOUSESHOW
_MOUSECURSOR 2
_WINDOWSIZELIMIT (100, 100)-(4000, 4000)
PRINT _DESKTOPWIDTH > 0; _DESKTOPHEIGHT > 0
x& = _SCREENX: y& = _SCREENY
PRINT _WINDOWHANDLE <> 0
f& = _WINDOWHASFOCUS
PRINT _SCREENICON
PRINT _FULLSCREEN
PRINT "done"
SYSTEM
