$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a single-line IF inside the branch of another, the ELSE the inner one's.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    For b = 0 To 1
        x = 0
        If a Then
            If b Then
                x = 1
            Else
                x = 2
            End If
        End If
        Print a; b; x
    Next
Next
System
