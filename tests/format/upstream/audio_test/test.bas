$Console
$ScreenHide
_Dest _Console

s = _SndOpen("foobar.mp3", "VOL,PAUSE,SETPOS"): _SndLoop s
_SndClose s

Print "Sound";

System
