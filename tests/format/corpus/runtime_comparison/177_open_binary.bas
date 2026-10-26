$Console:Only
' Runtime comparison: OPEN FOR BINARY (if supported)
Open "rt_177_bin.txt" For Binary As #1
Print #1, "ab"
Close #1
Open "rt_177_bin.txt" For Binary As #2
Dim s As String
s = Space$(4)
Get #2, , s
Print "["; s; "]"
Close #2
Kill "rt_177_bin.txt"
Print "done"
End
