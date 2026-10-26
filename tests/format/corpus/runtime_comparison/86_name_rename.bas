$Console:Only
' Runtime comparison: NAME (rename file)
Open "rt_86_old.txt" For Output As #1
Print #1, "content"
Close #1
Name "rt_86_old.txt" As "rt_86_new.txt"
Open "rt_86_new.txt" For Input As #2
Dim s As String
Line Input #2, s
Print "renamed:"; s
Close #2
Kill "rt_86_new.txt"
Print "done"
End
