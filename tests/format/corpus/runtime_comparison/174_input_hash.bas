$Console:Only
' Runtime comparison: INPUT # (numeric/string from file)
Open "rt_174_in.txt" For Output As #1
Print #1, "hello"
Print #1, 42
Print #1, 3.14
Close #1
Open "rt_174_in.txt" For Input As #2
Dim s As String, i As Long, d As Double
Input #2, s
Input #2, i
Input #2, d
Print s; i; d
Close #2
Kill "rt_174_in.txt"
Print "done"
End
