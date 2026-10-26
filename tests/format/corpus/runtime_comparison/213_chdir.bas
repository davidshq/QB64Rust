$Console:Only
' Runtime comparison: CHDIR then file open with relative path
On Error GoTo handler
MkDir "rt_213_dir"
ChDir "rt_213_dir"
Open "rt_213_f.txt" For Output As #1
Print #1, "ok"
Close #1
ChDir ".."
Open "rt_213_dir/rt_213_f.txt" For Input As #2
Dim s As String
Line Input #2, s
Print s
Close #2
Kill "rt_213_dir/rt_213_f.txt"
RmDir "rt_213_dir"
Print "done"
End
handler:
Print "err"; Err
Resume Next
