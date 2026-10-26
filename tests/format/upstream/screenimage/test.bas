$Console
$ScreenHide
_Dest _Console

$If WIN Then
$Else

' We can't do _SCREENIMAGE on Linux or Mac OS build agents, but we can still
' test that it compiles
PRINT -11;
SYSTEM

$End If

i& = _ScreenImage
Print i&;
System
