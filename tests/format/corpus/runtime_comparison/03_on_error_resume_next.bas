$Console:Only
' Runtime comparison 3: ON ERROR RESUME NEXT
' Trigger error, print ERR/ERL, resume
On Error GoTo handler
Print "before"
Error 5
Print "after error"
End

handler:
Print "handler: ERR="; Err; " ERL="; Erl
Resume Next
