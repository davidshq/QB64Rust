$Console:Only
' Runtime comparison: SUB with string param
Dim s As String
s = "hello"
Call printStr(s)
End

Sub printStr (x As String)
    Print "got:"; x
End Sub
