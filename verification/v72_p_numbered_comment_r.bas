$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a comment on a numbered line that a jump names.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    If a Then
        GoTo 20
    End If
    Print "zero"
    ' the target of the jump
    20 x = a
    y = a * 2
    Print x; y
Next
System
