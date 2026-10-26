$Console:Only
' Runtime comparison: EXIT DO
Dim n As Long
n = 0
Do
    Print n;
    n = n + 1
    If n >= 3 Then Exit Do
Loop
Print
Print "done"
End
