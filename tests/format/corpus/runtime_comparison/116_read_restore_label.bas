$Console:Only
' Runtime comparison: RESTORE label
Dim x As Long
Read x
Print "first:"; x
Restore second
Read x
Print "second block:"; x
End

first:
Data 1,2,3
second:
Data 10,20
