' Screen program (m4-coverage-review, design D5): the window built-ins that act on the program's own window or
' read a value of the machine, run. A value that differs between runs or machines (the window's place, the
' desktop's size, the focus) is printed as a property or not at all. So is _TITLE$: read back after _TITLE, the
' old compiler's build gave five bytes that differ from run to run, not the title (on the runner's hidden desktop).
Screen 12
_Title "w01 window"
t$ = _Title$
_MouseHide
_MouseShow
_MouseCursor 2
_WindowSizeLimit (100, 100)-(4000, 4000)
Print _DesktopWidth > 0; _DesktopHeight > 0
x& = _ScreenX: y& = _ScreenY
Print _WindowHandle <> 0
f& = _WindowHasFocus
Print _ScreenIcon
Print _FullScreen
Print "done"
System
