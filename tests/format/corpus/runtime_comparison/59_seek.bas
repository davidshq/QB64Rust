$Console:Only
' Runtime comparison: SEEK
Open "rt_59_seek.txt" For Output As #1
Print #1, "abc"
Print #1, "def"
Close #1
Open "rt_59_seek.txt" For Input As #2
Dim s As String
Line Input #2, s
Print "pos after read:"; Seek(2)
Line Input #2, s
Print "pos after read2:"; Seek(2)
Close #2
Kill "rt_59_seek.txt"
Print "done"
End
