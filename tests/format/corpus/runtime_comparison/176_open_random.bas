$Console:Only
' Runtime comparison: OPEN FOR RANDOM (if supported)
On Error GoTo handler
Open "rt_176_ran.txt" For Random As #1 Len = 10
Print "open random ok"
Close #1
Kill "rt_176_ran.txt"
Print "done"
End
handler:
Print "err"; Err
Resume Next
