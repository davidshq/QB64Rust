$Console
' Slice (m4-qb64pe-builtins, task 4.4): _CONSOLE OFF and ON in a $CONSOLE program (verification\v53_a_window),
' _EXIT with no request to end, and the drop built-ins with nothing dropped. _SCREENMOVE and the dialogs (task 4.3)
' as code that is not reached: they are never run.
_Dest _Console
k = 0
If k = 1 Then
    _ScreenMove _Middle
    _MessageBox "t", "m", "info"
    Print _MessageBox("t", "m", "yesno", "question", 0)
    Print _OpenFileDialog$("t"); _SaveFileDialog$("t"); _SelectFolderDialog$("t")
End If
Print "a"
_Console Off
Print "b"
_Console On
Print "c"
Print _Exit
_AcceptFileDrop
Print _AcceptFileDrop; _TotalDroppedFiles
Print "["; _DroppedFile$; "]["; _DroppedFile(1); "]"
_FinishDrop
_AcceptFileDrop Off
Print _AcceptFileDrop
System
