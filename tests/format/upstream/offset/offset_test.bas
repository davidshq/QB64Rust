$Console:Only
Option _Explicit

Dim myArray(1 To 10) As Long
Dim i As Long

For i = LBound(myArray) To UBound(myArray)
    myArray(i) = 2 ^ i
Next i

PrintArray myArray()

Dim arrMem As _MEM: arrMem = _Mem(myArray())

For i = 0 To (arrMem.SIZE \ _SIZE_OF_LONG) - 1
    _MemPut arrMem, arrMem.OFFSET + (i * _SIZE_OF_LONG), i + 1 As Long
Next i

_MemFree arrMem

PrintArray myArray()

System

Sub PrintArray (arr() As Long)
    Dim i As Long

    Print "Array("; LBound(arr); "To"; UBound(arr); ") = {";
    For i = LBound(arr) To UBound(arr)
        Print arr(i); _IIf(i < UBound(arr), ",", "");
    Next i
    Print "}"
End Sub
