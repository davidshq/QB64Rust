$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a built-in statement's name and a colon, a label, and a SUB's name and a colon.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
Cls
Print "start"
n = 1
top: x = 9
b = 9
Print x; b; n
DrawBoard: Print "Ready"
x = 1
Print x
System
Sub DrawBoard
    Print "board"
End Sub
