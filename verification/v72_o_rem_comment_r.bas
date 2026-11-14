$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a REM comment after a single-line IF.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    x = 0
    Rem first turn
    If a Then
        x = 1
    End If
    Print x
Next
System
