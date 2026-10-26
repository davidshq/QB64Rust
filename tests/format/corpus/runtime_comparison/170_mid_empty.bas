$Console:Only
' Runtime comparison: MID$ on empty string, start > LEN
Dim s As String
s = ""
Print "len:"; Len(s)
Print "mid(1,1):"; Mid$(s, 1, 1)
s = "ab"
Print "mid(5,1):"; Mid$(s, 5, 1)
Print "mid(1,0):"; Mid$(s, 1, 0)
End
