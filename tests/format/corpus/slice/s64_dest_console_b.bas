' Slice (m2-console-screenhide, group 3): _DEST, _SOURCE, _CONSOLE and _ECHO in a program without a window. The
' console is the destination and the source from the first $CONSOLE:ONLY; _DEST 0 takes the output away from
' standard output, and $CONSOLE:ONLY a second time selects the console again where it stands. _ECHO stands above
' the first $CONSOLE:ONLY too.
_Echo "echo above"
$Console:Only
Print "console 1"; _Dest; _Console; _Source
_Echo "echo 1"
_Dest 0
Print "window"
_Echo "echo 2:" + Str$(_Dest)
$Console:Only
Print "console 2"; _Dest
_Dest 0: _Source 0
Print "window"; _Source
_Dest _Console
Print "console 3"; _Dest; _Source
System
