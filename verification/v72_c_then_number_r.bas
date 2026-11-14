$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): THEN and ELSE followed by a line number.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    If a Then
        GoTo 100
    Else
        GoTo 200
    End If
    100 Print "then"
    GoTo 300
    200 Print "else"
    300 Print "end"
Next
System
