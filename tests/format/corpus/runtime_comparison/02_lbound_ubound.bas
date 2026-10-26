$Console:Only
' Runtime comparison 2: LBOUND/UBOUND
' DIM with explicit bounds, print bounds and iterate
Dim arr(1 To 5) As Long
Dim i As Long
Print "lbound: "; LBound(arr)
Print "ubound: "; UBound(arr)
For i = LBound(arr) To UBound(arr)
    arr(i) = i * 10
Next i
For i = LBound(arr) To UBound(arr)
    Print arr(i);
Next i
Print
End
