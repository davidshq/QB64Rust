$Console:Only
' Runtime comparison: MKDIR, then RMDIR
On Error GoTo handler
MkDir "rt_214_dir"
Print "mkdir ok"
RmDir "rt_214_dir"
Print "rmdir ok"
Print "done"
End
handler:
Print "err"; Err
Resume Next
