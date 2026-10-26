Option _Explicit
DefLng A-Z
$Console:Only
Dim Debug As Long
Debug = -1

'$Include:'../../../source/global/constants.bas'
'$Include:'../../../source/utilities/type.bi'
sp = "@" ' Makes the output readable

Type TestCase
    args As String
    results As String
End Type

Dim tests(6) As TestCase

tests(1).args = "2"
tests(1).results = MKL$(-1) + MKL$(0) + MKL$(0) + MKL$(0)

tests(2).args = "2" + sp + "," + sp + "3"
tests(2).results = MKL$(-1) + MKL$(-1) + MKL$(0) + MKL$(0)

' (2, foo(3, 2) ) -  includes two function arguments, with one of them being a function call
tests(3).args = "2" + sp + "," + sp + _
                    "foo" + sp + "(" + sp + "3" + sp + "," + sp + "2" + sp + ")"
tests(3).results = MKL$(-1) + MKL$(-1) + MKL$(0) + MKL$(0) + MKL$(0)

' Trailing comma at the end of argument list
tests(4).args = "2" + sp + "," + sp + "3" + sp + ","
tests(4).results = MKL$(-1) + MKL$(-1) + MKL$(0) + MKL$(0)

tests(5).args = "2" + sp + "," + sp + "," + sp + "3"
tests(5).results = MKL$(-1) + MKL$(0) + MKL$(-1) + MKL$(0)

tests(6).args = "2" + sp + "," + sp + ","
tests(6).results = MKL$(-1) + MKL$(0) + MKL$(0) + MKL$(0)

ReDim provided(10) As Long, i As Long
For i = 1 To UBound(tests)
    argStringToArray tests(i).results, provided()

    Print "Test"; i; ", Args: "; tests(i).args

    Dim k As Long
    For k = 1 To UBound(provided)
        Dim result As Long

        result& = hasFunctionElement(tests(i).args, k)

        Print "    Expected:"; provided(k); ", Actual"; result&;
        If provided(k) = result& Then
            Print " PASS!"
        Else
            Print " FAIL!"
        End If
    Next

    Print
Next

System

'$Include:'../../../source/utilities/elements.bas'

Sub argStringToArray (argString As String, provided() As Long)
    ReDim provided(Len(argString) / 4) As Long, i As Long

    For i = 1 To UBound(provided)
        provided(i) = CVL(Mid$(argString, (i - 1) * 4 + 1, 4))
    Next
End Sub

Function argStringPrint$ (argString As String)
    Dim res As String, i As Long

    res$ = ""

    res$ = Str$(CVL(Mid$(argString, 1, 4)))
    For i = 2 To Len(argString) / 4
        res$ = res$ + ", " + Str$(CVL(Mid$(argString, (i - 1) * 4 + 1, 4)))
    Next i

    argStringPrint$ = res$
End Function
