$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _TRUE
    System
$End If

$Console
_Dest _Console

Print _DesktopWidth > 0
System
