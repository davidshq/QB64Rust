$Console:Only
' Runtime comparison: OPEN FOR APPEND
Open "rt_53_append.txt" For Output As #1
Print #1, "line1"
Close #1
Open "rt_53_append.txt" For Append As #1
Print #1, "line2"
Close #1
Open "rt_53_append.txt" For Input As #2
Dim s As String
Line Input #2, s
Print "first:"; s
Line Input #2, s
Print "second:"; s
Close #2
Kill "rt_53_append.txt"
Print "done"
End
