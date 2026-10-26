$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _TRUE
    System
$End If

$Console
_Dest _Console

_WindowSizeLimit
_WindowSizeLimit (320, 240)
_WindowSizeLimit -(800, 600)
_WindowSizeLimit (320, 240)-(800, 600)

Print _TRUE
System
