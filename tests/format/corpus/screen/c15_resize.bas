$Resize:On
' Screen program (m4-qb64pe-builtins, task 2.1): $RESIZE:ON with no resize by the user: _RESIZE, _RESIZEWIDTH and
' _RESIZEHEIGHT after half a second, again after the read, and after the statement _RESIZE OFF.
' Where the window opens at a size other than 640 x 400 (a monitor's content scale), libqb sets it to 640 x 400 and
' its size callback leaves one resize event of 640 x 400 (glut-emu.cpp, window-gui.cpp GLUT_RESIZE_FUNC); whether
' it does depends on the desktop, so either outcome passes.
_Delay .5
r = _Resize: w = _ResizeWidth: h = _ResizeHeight
Print (r = 0 And w = 0 And h = 0) Or (r = -1 And w = 640 And h = 400)
Print _Resize; _ResizeWidth = w; _ResizeHeight = h
_Resize Off
Print _Resize; _ResizeWidth = w; _ResizeHeight = h
System
