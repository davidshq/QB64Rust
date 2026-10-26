$Console:Only
' Runtime comparison: SQR negative (error path if any)
On Error GoTo handler
Print Sqr(-1)
Print "no error"
End
handler:
Print "err"; Err; "erl"; Erl
Resume Next
