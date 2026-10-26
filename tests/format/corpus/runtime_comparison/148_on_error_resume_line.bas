$Console:Only
' Runtime comparison: ON ERROR GOTO ... RESUME <label>
On Error GoTo handler
Print "before"
Error 6
Print "after"
End

handler:
Print "handler: ERR="; Err; " ERL="; Erl
Resume afterLabel

afterLabel:
Print "resumed at afterLabel"
End
