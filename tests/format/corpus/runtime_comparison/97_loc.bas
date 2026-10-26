$Console:Only
' Runtime comparison: LOC
Open "rt_97_loc.txt" For Output As #1
Print #1, "ab"
Close #1
Open "rt_97_loc.txt" For Input As #2
Dim s As String
Line Input #2, s
Print "loc after read:"; Loc(2)
Close #2
Kill "rt_97_loc.txt"
Print "done"
End
