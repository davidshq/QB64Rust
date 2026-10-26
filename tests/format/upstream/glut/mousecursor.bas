$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print _TRUE
    System
$End If

$Console
_Dest _Console

Dim img As Long: img = _NewImage(32, 32, 32)

_MouseCursor img
_MouseCursor img, (0, 0)
_MouseCursor img, (16, 16)
_MouseShow

_FreeImage img

Print _TRUE
System
