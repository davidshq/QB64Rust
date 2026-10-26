$Console:Only
' Runtime comparison: Nested WHILE/WEND
Dim i As Long, j As Long
i = 0
While i < 2
    j = 0
    While j < 2
        Print i; j
        j = j + 1
    Wend
    i = i + 1
Wend
Print "done"
End
