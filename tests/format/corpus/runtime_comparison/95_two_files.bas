$Console:Only
' Runtime comparison: two files open
Open "rt_95_a.txt" For Output As #1
Open "rt_95_b.txt" For Output As #2
Print #1, "file1"
Print #2, "file2"
Close #1
Close #2
Open "rt_95_a.txt" For Input As #1
Open "rt_95_b.txt" For Input As #2
Dim s As String
Line Input #1, s
Print "a:"; s
Line Input #2, s
Print "b:"; s
Close #1
Close #2
Kill "rt_95_a.txt"
Kill "rt_95_b.txt"
Print "done"
End
