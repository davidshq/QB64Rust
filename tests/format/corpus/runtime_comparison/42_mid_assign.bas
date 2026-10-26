$Console:Only
' Runtime comparison: MID$ assignment
Dim s As String
s = "abcdef"
Mid$(s, 2, 2) = "XY"
Print "mid assign:"; s
End
