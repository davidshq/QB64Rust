$Console:Only
' Runtime comparison: FREEFILE
Dim f As Long
f = FreeFile
Print "freefile:"; f
Open "rt_54_f.txt" For Output As #f
Print #f, "test"
Close #f
Kill "rt_54_f.txt"
Print "done"
End
