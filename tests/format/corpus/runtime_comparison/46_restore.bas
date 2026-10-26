$Console:Only
' Runtime comparison: RESTORE
Dim x As Long
Read x
Print "first:"; x
Restore
Read x
Print "after restore:"; x
End

Data 99
