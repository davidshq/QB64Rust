$Console:Only
' Runtime comparison: RESTORE without label (next READ from start)
Dim a As Long, b As Long
Read a
Read b
Print a; b
Restore
Read a
Read b
Print a; b
Data 1,2
End
