' Screen program (m4-coverage-review, design D5): _DEPTHBUFFER on a hardware image, run. What it does shows only
' in what OpenGL draws, outside the state the screen check reads; the program goes on after it.
Screen 12
h& = _CopyImage(_NewImage(8, 8, 32), 33)
Print h& < -1
_DepthBuffer On , h&
_DepthBuffer _Clear , h&
_DepthBuffer Off , h&
Print "done"
System
