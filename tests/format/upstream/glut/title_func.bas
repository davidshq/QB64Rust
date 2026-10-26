$If WINDOWS OR MACOSX Then
    ' GLFW cannot create a window with OpenGL context in a macOS and Windows GitHub runner
    $Console:Only
    Print "Title: "
    System
$End If

$Console
_Dest _Console

Print "Title: "; _Title$
System
