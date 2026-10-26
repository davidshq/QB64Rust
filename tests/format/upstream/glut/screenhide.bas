$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print "Got Past ScreenHide!"
    System
$End If

$Console
_Dest _Console

_ScreenHide
Print "Got Past ScreenHide!"
System
