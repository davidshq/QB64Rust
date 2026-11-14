$Console:Only
' Verification (OpenSpec change format-statement-lines, design D9): statements packed with colons, and a comment.
' The _r file is the form the formatter writes with OneStatementPerLine and BlockIf, _b with BlockIf alone.
For a = 0 To 1
    ' three statements
    x = a
    y = a + 1
    Print x; y
Next
System
