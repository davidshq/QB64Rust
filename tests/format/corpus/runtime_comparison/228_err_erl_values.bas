$Console:Only
' Runtime comparison: PRINT ERR, ERL after different errors
On Error GoTo handler
Print "before"
Error 11
Print "after"
End
handler:
Print "err:"; Err; "erl:"; Erl
Resume Next
