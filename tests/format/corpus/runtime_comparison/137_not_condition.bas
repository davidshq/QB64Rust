$Console:Only
' Runtime comparison: NOT in condition
Dim x As Long
x = 0
If Not x Then Print "not zero"
x = 1
If Not (x = 0) Then Print "x not zero"
End
