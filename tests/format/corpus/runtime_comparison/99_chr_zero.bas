$Console:Only
' Runtime comparison: CHR$(0)
Dim s As String
s = Chr$(0) + "x" + Chr$(0)
Print "len chr0 x chr0:"; Len(s)
Print "asc pos 1:"; Asc(s, 1)
Print "asc pos 2:"; Asc(s, 2)
End
