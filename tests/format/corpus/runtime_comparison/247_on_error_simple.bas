$Console:Only
' Runtime comparison: ON ERROR GOTO, PRINT ERR/ERL, RESUME NEXT (no DECLARE LIBRARY)
On Error GoTo eh
Print "before"
Error 5
Print "after error"
End
eh:
Print "handler"; Err; Erl
Resume Next
