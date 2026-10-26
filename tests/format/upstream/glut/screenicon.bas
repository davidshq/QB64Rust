$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _FALSE
    System
$End If

$Console
_Dest _Console

Print _ScreenIcon <> 0
System
