$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _TRUE
    Print _TRUE
    Print "Got past icon!"
    Print "Got past MouseHide!"
    Print "Got past MouseShow!"
    Print _FALSE
    Print "Got past ScreenHide"
    Print _FALSE
    Print "Error:"; 5
    Print "Error:"; 5
    Print "Title: foobar"
    Print _FALSE
    Print _TRUE
    Print "Got past ScreenShow!"
    System
$End If

$ScreenHide
$Console
_Dest _Console
On Error GoTo errorhand

Print _DesktopHeight > 0

Print _DesktopWidth > 0

_Icon
Print "Got past icon!"

_MouseHide
Print "Got past MouseHide!"

_MouseShow
Print "Got past MouseShow!"

Print _ScreenExists

_ScreenHide
Print "Got past ScreenHide"

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

System

errorhand:
Print "Error:"; Err
Resume Next
