$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): IF and GOTO without THEN.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    If a Then
        GoTo 100
    End If
    Print "no"
    100 Print "end"
Next
System
