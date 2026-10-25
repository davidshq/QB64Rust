' Screen program (m4-coverage-review, design D5): _DEPTHBUFFER on a hardware image, run. What it does shows only
' in what OpenGL draws, outside the state the screen check reads; the program goes on after it.
SCREEN 12
h& = _COPYIMAGE(_NEWIMAGE(8, 8, 32), 33)
PRINT h& < -1
_DEPTHBUFFER ON, h&
_DEPTHBUFFER _CLEAR, h&
_DEPTHBUFFER OFF, h&
PRINT "done"
SYSTEM
