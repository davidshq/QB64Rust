$Console:Only
' Runtime comparison: AND/OR in condition
Dim a As Long, b As Long
a = 1
b = 1
If a = 1 And b = 1 Then Print "both"
If a = 0 Or b = 1 Then Print "or"
End
