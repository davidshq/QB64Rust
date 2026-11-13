$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a single-line IF with ELSE and colons in both branches.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    If a Then
        x = 1: y = 2
    Else
        x = 3: y = 4
    End If
    Print x; y
Next
System
