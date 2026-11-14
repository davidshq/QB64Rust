$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): statements after ELSEIF ... THEN and after the ELSE of a block IF.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 2
    c = 0
    If a = 0 Then
        x = 1
    ElseIf a = 1 Then
        x = 2
        c = 3
    Else
        x = 3
    End If
    Print x; c
Next
System
