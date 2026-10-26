$Console:Only
' Runtime comparison: EOF
Open "rt_60_eof.txt" For Output As #1
Print #1, "one"
Close #1
Open "rt_60_eof.txt" For Input As #2
Dim s As String
Line Input #2, s
Print "eof after 1:"; EOF(2)
Line Input #2, s
Print "eof after 2:"; EOF(2)
Close #2
Kill "rt_60_eof.txt"
Print "done"
End
