$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a single-line IF with an empty THEN branch.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    x = 0
    If a Then
    Else
        x = 4
    End If
    Print x
Next
System
