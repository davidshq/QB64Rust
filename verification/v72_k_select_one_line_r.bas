$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a whole SELECT CASE on one line.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    Select Case a
        Case 0
            b = 5
        Case Else
            b = 6
    End Select
    Print b
Next
System
