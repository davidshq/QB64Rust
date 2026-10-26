$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _TRUE
    Print _TRUE
    Print _TRUE
    Print _TRUE
    System
$End If

$Console
_Dest _Console

Print _WindowHandle <> 0
Print _WindowHandle(0) <> 0
Print _WindowHandle(1) <> 0
Print _WindowHandle(2) <> 0

System
