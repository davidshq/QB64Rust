$Console:Only
' Runtime comparison: TYPE with fixed-length string (same as 57)
Type TStr
    s As String * 5
End Type
Dim t As TStr
t.s = "hi"
Print "["; t.s; "]"
t.s = "hello"
Print "["; t.s; "]"
End
