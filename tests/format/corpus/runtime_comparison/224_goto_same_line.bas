$Console:Only
' Runtime comparison: GOTO same line (no-op or loop)
Dim n As Long
n = 0
label1:
n = n + 1
If n < 2 Then GoTo label1
Print "n:"; n
End
