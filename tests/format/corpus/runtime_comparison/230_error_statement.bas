$Console:Only
' Runtime comparison: ERROR 5 (simulate error)
On Error GoTo handler
Print "before"
Error 5
Print "after"
End
handler:
Print "err:"; Err; "erl:"; Erl
Resume Next
