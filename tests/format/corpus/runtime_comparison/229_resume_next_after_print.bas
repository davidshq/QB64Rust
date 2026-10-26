$Console:Only
' Runtime comparison: RESUME NEXT then use variable
On Error GoTo handler
Dim x As Long
x = 1 / 0
Print "x:"; x
End
handler:
Print "handler"
Resume Next
