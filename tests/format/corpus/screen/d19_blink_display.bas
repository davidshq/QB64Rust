' Screen (m4-builtins-remaining, task 3.2): _BLINK OFF with a line in a blinking colour, _BLINK as a function, the
' display statements in each form and the display functions in a text window (verification\v58_s05, s06, f14): the
' values that are the machine's (the refresh rate, the lock keys) as comparisons.
Screen 0
_Blink Off
Color 20, 1: Print "x"
Color 7, 0
Print _Blink
_Blink On: Print _Blink
_Blink Off
_GLRender _Behind: _GLRender _OnTop: _GLRender _Behind
_DisplayOrder _Software , _Hardware: _DisplayOrder _Software
_AllowFullScreen _Stretch , _Smooth: _AllowFullScreen _Off: _AllowFullScreen
_FPS 30: _FPS _Auto
Print _ScaledWidth; _ScaledHeight; _Smooth; _DesktopRefreshRate > 0; _Display; _Display = _Dest
c = _CapsLock: n = _NumLock: s = _ScrollLock
Print c = 0 Or c = -1; n = 0 Or n = -1; s = 0 Or s = -1
System
