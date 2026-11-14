$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a statement, a colon and the header of a block IF.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    b = 0
    x = 1
    If a Then
        b = 1
    End If
    Print x; b
Next
System
