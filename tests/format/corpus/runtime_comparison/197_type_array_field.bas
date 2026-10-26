$Console:Only
' Runtime comparison: TYPE with array field
Type WithArray
    id As Long
    arr(1 TO 3) As Long
End Type
Dim w As WithArray
w.id = 1
w.arr(1) = 10
w.arr(2) = 20
w.arr(3) = 30
Print w.id; w.arr(1); w.arr(2); w.arr(3)
End
