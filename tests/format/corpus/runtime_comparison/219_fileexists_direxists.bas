$Console:Only
' Runtime comparison: _FILEEXISTS, _DIREXISTS (if supported)
On Error GoTo handler
Open "rt_219_f.txt" For Output As #1
Close #1
Print _FileExists("rt_219_f.txt")
Print _FileExists("rt_219_nonexist.txt")
MkDir "rt_219_d"
Print _DirExists("rt_219_d")
RmDir "rt_219_d"
Kill "rt_219_f.txt"
Print "done"
End
handler:
Print "err"; Err
Resume Next
