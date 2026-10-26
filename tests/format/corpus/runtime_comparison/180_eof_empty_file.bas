$Console:Only
' Runtime comparison: EOF on empty file
Open "rt_180_empty.txt" For Output As #1
Close #1
Open "rt_180_empty.txt" For Input As #2
Print "eof:"; EOF(2)
Close #2
Kill "rt_180_empty.txt"
Print "done"
End
