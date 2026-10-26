$Console:Only
' Runtime comparison: ENVIRON$("VAR") - output may differ by environment
Dim s As String
s = Environ$("PATH")
Print "PATH len:"; Len(s)
s = Environ$("NONEXISTENT_VAR_12345")
Print "nonexist len:"; Len(s)
End
