$Console:Only
' Runtime comparison: LOF on binary file
Open "rt_182_bin.txt" For Binary As #1
Print #1, "12345";
Print "lof:"; LOF(1)
Close #1
Open "rt_182_bin.txt" For Binary As #2
Print "lof read:"; LOF(2)
Close #2
Kill "rt_182_bin.txt"
Print "done"
End
