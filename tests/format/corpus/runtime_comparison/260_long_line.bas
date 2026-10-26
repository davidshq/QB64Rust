$Console:Only
' Runtime comparison: Single PRINT with very long string
Dim s As String
s = String$(200, "x")
Print s
Print "len:"; Len(s)
End
