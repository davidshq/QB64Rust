$Console:Only
' Runtime comparison 5: File I/O handle management
' OPEN, write, close, reopen read, LINE INPUT, close
Open "runtime_comparison_test.txt" For Output As #1
Print #1, "line1"
Print #1, "line2"
Close #1

Open "runtime_comparison_test.txt" For Input As #2
Dim s As String
Line Input #2, s
Print "first: "; s
Line Input #2, s
Print "second: "; s
Close #2

Kill "runtime_comparison_test.txt"
Print "done"
End
