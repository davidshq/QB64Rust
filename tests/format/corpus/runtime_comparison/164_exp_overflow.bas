$Console:Only
' Runtime comparison: EXP overflow / LOG domain
On Error GoTo handler
Print Exp(700)
Print "no overflow"
End
handler:
Print "err"; Err; "erl"; Erl
Resume Next
