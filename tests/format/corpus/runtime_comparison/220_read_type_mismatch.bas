$Console:Only
' Runtime comparison: READ into wrong type (string vs number)
On Error GoTo handler
Dim x As Long, s As String
Read x
Print "x:"; x
Read s
Print "s:"; s
Data 42,hello
End
handler:
Print "err"; Err; "erl"; Erl
Resume Next
