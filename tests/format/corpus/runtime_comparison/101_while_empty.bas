$Console:Only
' Runtime comparison: WHILE false (body not executed)
Dim n As Long
n = 5
While n < 3
    Print "never"
Wend
Print "done"
End
