$Console:Only
' Runtime comparison: long string concat
Dim s As String
s = "a" + "b" + "c" + "d" + "e"
Print "concat:"; s
Print "len:"; Len(s)
End
