$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a whole DO ... LOOP UNTIL on one line.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    n = 0
    Do
        n = n + 1
    Loop Until n >= 3 + a
    Print n
Next
System
