$Console:Only
' Runtime comparison: OPEN FOR OUTPUT, WRITE
Open "rt_52_out.txt" For Output As #1
Write #1, "a", 1, 2.5
Close #1
Open "rt_52_out.txt" For Input As #2
Dim s As String, i As Long, d As Double
Input #2, s, i, d
Print "read:"; s; i; d
Close #2
Kill "rt_52_out.txt"
Print "done"
End
