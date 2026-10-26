$Console:Only
' Runtime comparison: GET # / PUT # (binary record I/O)
Open "rt_175_bin.txt" For Binary As #1
Dim s As String
s = "xy"
Put #1, , s
Close #1
Open "rt_175_bin.txt" For Binary As #2
s = "  "
Get #2, , s
Print "["; s; "]"
Close #2
Kill "rt_175_bin.txt"
Print "done"
End
