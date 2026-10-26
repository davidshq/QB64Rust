$Console
$ScreenHide
_Dest _Console

$If LINUX Then

' FIXME: Linux works but the test gives some kind of freeglut error, presumably
'        some kind of rece condition
print -1;

$Else

    Print _Devices > 1;
    _Delay 1

$End If

System
