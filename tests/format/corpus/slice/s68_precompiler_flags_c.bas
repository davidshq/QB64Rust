$ScreenHide
' Slice (m2-console-screenhide, group 6): the precompiler flags, third program. The only $CONSOLE:ONLY stands
' inside the branch its own flag switches off. _CONSOLE_ is 2 and the program has a console, by the flag; the
' line that would take the window away is not compiled in the last round, so the program keeps its (hidden)
' window: the destination is the window (0, not the console's handle), a PRINT goes there and is not seen, and
' _ECHO writes to the console.
$If _CONSOLE_ = 0 Then
    $CONSOLE:ONLY
$End If
Print "printed to the window"
$If _CONSOLE_ = 2 Then
    _Echo "flag 2"
$ElseIf _CONSOLE_ = 1 Then
    _ECHO "flag 1"
$Else
    _ECHO "flag 0"
$End If
_Echo "destination:" + Str$(_Dest) + Str$(_Console)
_Dest _Console
Print "after _DEST _CONSOLE"
System
