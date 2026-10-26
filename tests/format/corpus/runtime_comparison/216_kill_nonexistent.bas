$Console:Only
' Runtime comparison: KILL on nonexistent file (error code)
On Error GoTo handler
Kill "rt_216_nonexistent_xyz.txt"
Print "no error"
End
handler:
Print "err"; Err; "erl"; Erl
Resume Next
