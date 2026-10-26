' Slice (m2-console-screenhide, group 4): the window built-ins, compiled and never run. The shape of the upstream
' glut tests: a program without a window prints a line and leaves, and below its SYSTEM stands every window
' statement and function in every form, as code that is compiled, linked and not reached. What each does in a
' window is checked from M3 on; the call each compiles to is compared by tests\callsite.
$Console:Only
Dim l As Long, i As Integer, q As _Integer64, sg As Single, f As Double, o As _Unsigned _Offset
Dim s As String, fx As String * 4
Print "window built-ins: compiled"
System

$Console
_Dest _Console

_ScreenHide
_ScreenShow
_ScreenIcon
Call _ScreenHide
Call _ScreenShow
Call _ScreenIcon

_Title "title"
_Title s
_Title fx
_Title s + "!"
Call _Title(s)

_Icon
_Icon l
_Icon l, l
_Icon i, sg
_Icon 1.6, q

_MouseShow
_MouseShow "LINK"
_MouseShow s
_MouseHide
_MouseHide _Hide
_MouseHide _Disable
_MouseCursor l
_MouseCursor l, (l, l)
_MouseCursor f, (i, q)
_MouseCursor l + 1, (16, 16)

_WindowSizeLimit
_WindowSizeLimit (320, 240)
_WindowSizeLimit -(800, 600)
_WindowSizeLimit (320, 240)-(800, 600)
_WindowSizeLimit (l, i)-(sg, f)

l = _NewImage(l, l)
l = _NewImage(640, 480, 32)
l = _NewImage(i, sg, f)
l = _NewImage(_DesktopWidth, _DesktopHeight, 32)
_FreeImage
_FreeImage l
_FreeImage sg
_FreeImage _NewImage(8, 8, 32)

l = _ScreenExists
l = _ScreenIcon
l = _DesktopWidth
l = _DesktopHeight
l = _ScreenX
l = _ScreenY
l = _WindowHasFocus
s = _Title$
f = _DesktopWidth / 2
l = _DesktopWidth - _ScreenX
l = Not _ScreenExists
s = _Title$ + Str$(_ScreenY) + Hex$(_ScreenX) + Str$(_ScreenExists)
If _ScreenExists Then Print _ScreenIcon; _DesktopHeight; _ScreenExists; _Title$
If _WindowHasFocus = 0 Then Print _ScreenX; _ScreenY
Select Case _ScreenExists
    Case -1
        Print "exists"
End Select

o = _WindowHandle
o = _WindowHandle(l)
q = _WindowHandle(1)
f = _WindowHandle / 3
s = Str$(_WindowHandle)
Print _WindowHandle <> 0
Print _WindowHandle(1) <> 0
Print _WindowHandle
inSub
System

Sub inSub
    _Title "in the sub"
    _MouseHide
    Print _ScreenExists; _ScreenX
End Sub
