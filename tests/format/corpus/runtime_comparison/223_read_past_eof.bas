$Console:Only
' Runtime comparison: READ past last DATA (error)
On Error GoTo handler
Dim x As Long
Read x
Print x
Read x
Print "second:"; x
Data 1
End
handler:
Print "err"; Err; "erl"; Erl
Resume Next
