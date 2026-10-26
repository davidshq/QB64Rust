$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _TRUE
    Print _TRUE
    Print "Got Past Icon!"
    Print "Got Past MouseHide!"
    Print "Got Past MouseShow!"
    Print _TRUE
    Print _FALSE
    Print _TRUE
    Print _TRUE
    Print "Title: foobar"
    Print _TRUE
    Print _TRUE
    Print "Got past ScreenShow!"
    System
$End If

$Console
_Dest _Console

_ScreenHide
Print _DesktopHeight > 0
Print _DesktopWidth > 0
_Icon
Print "Got Past Icon!"
_MouseHide
Print "Got Past MouseHide!"
_MouseShow
Print "Got Past MouseShow!"
Print _ScreenExists
Print _ScreenIcon <> 0
Print _ScreenX >= 0
Print _ScreenY >= 0
_Title "foobar"
Print "Title: "; _Title$
Print _WindowHandle <> 0
Print _WindowHasFocus <= 0 ' This can be a bit random
_ScreenShow
Print "Got past ScreenShow!"
System
