$Console:Only
' Runtime comparison: LOF
Open "rt_96_lof.txt" For Output As #1
Print #1, "hello"
Close #1
Open "rt_96_lof.txt" For Input As #2
Print "lof:"; LOF(2)
Close #2
Kill "rt_96_lof.txt"
Print "done"
End
