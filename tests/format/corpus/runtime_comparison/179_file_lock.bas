$Console:Only
' Runtime comparison: LOCK / UNLOCK (if supported)
On Error GoTo handler
Open "rt_179_lk.txt" For Output As #1
Lock #1
Print #1, "x"
Unlock #1
Close #1
Kill "rt_179_lk.txt"
Print "done"
End
handler:
Print "err"; Err
Resume Next
