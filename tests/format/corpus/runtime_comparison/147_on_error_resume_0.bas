$Console:Only
' Runtime comparison: ON ERROR GOTO ... RESUME 0 (restart line)
' RESUME 0 re-executes the line that caused the error
On Error GoTo handler
Print "before"
x = 1
Error 5
Print "after"
End

handler:
Print "handler: ERR="; Err; " ERL="; Erl
Resume 0
