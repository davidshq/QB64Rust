$Console:Only
' Runtime comparison: WRITE # (comma-quoted output)
Open "rt_178_wr.txt" For Output As #1
Write #1, "a,b", 1, 2.5
Write #1, "line2"
Close #1
Open "rt_178_wr.txt" For Input As #2
Dim s As String, i As Long, d As Double
Input #2, s, i, d
Print "1:"; s; i; d
Line Input #2, s
Print "2:"; s
Close #2
Kill "rt_178_wr.txt"
Print "done"
End
