$Console:Only
' Runtime comparison: FREEFILE with multiple OPENs
Dim f1 As Long, f2 As Long
f1 = FreeFile
Open "rt_183_a.txt" For Output As #f1
f2 = FreeFile
Open "rt_183_b.txt" For Output As #f2
Print "f1:"; f1; "f2:"; f2
Print #f1, "a"
Print #f2, "b"
Close #f1
Close #f2
Kill "rt_183_a.txt"
Kill "rt_183_b.txt"
Print "done"
End
