$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a statement, a colon and a single-line IF with two statements.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    b = 0
    x = 1: If a Then
        b = 1: x = 2
    End If
    Print x; b
Next
System
