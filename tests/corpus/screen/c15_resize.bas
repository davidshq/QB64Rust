$RESIZE:ON
' Screen program (m4-qb64pe-builtins, task 2.1): $RESIZE:ON with no resize by the user: _RESIZE, _RESIZEWIDTH and
' _RESIZEHEIGHT after half a second, again after the read, and after the statement _RESIZE OFF.
' Where the window opens at a size other than 640 x 400 (a monitor's content scale), libqb sets it to 640 x 400 and
' its size callback leaves one resize event of 640 x 400 (glut-emu.cpp, window-gui.cpp GLUT_RESIZE_FUNC); whether
' it does depends on the desktop, so either outcome passes.
_DELAY .5
r = _RESIZE: w = _RESIZEWIDTH: h = _RESIZEHEIGHT
PRINT (r = 0 AND w = 0 AND h = 0) OR (r = -1 AND w = 640 AND h = 400)
PRINT _RESIZE; _RESIZEWIDTH = w; _RESIZEHEIGHT = h
_RESIZE OFF
PRINT _RESIZE; _RESIZEWIDTH = w; _RESIZEHEIGHT = h
SYSTEM
