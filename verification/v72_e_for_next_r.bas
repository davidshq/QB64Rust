$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): a whole FOR ... NEXT on one line.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 1 To 2
    For i = 1 To 3
        Print i * a;
    Next
    Print
Next
System
