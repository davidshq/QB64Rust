$Console:Only
$Unstable:TypeFields
Option _Explicit

' A _Dynamic AS STRING member array is handed straight to an ordinary
' items() AS STRING parameter, so the descriptor payload must match that of an
' ordinary STRING array.

Type Leaf
    labels(1) _Dynamic As String
    count As Long
End Type

Dim work As Leaf
ReDim work.labels(1 TO 3)
work.labels(1) = "red"
work.labels(2) = "green"
work.labels(3) = "blue"
work.count = 3

AssertString "direct 1", work.labels(1), "red"
AssertString "direct 3", work.labels(3), "blue"

ReadBack work.labels()
WriteThrough work.labels()

AssertString "after write 1", work.labels(1), "RED"
AssertString "after write 3", work.labels(3), "BLUE"
AssertBool "sibling member intact", work.count = 3

System

Sub ReadBack (items() As String)
    AssertString "param lbound", items(LBound(items)), "red"
    AssertString "param 2", items(2), "green"
    AssertString "param ubound", items(UBound(items)), "blue"
End Sub

Sub WriteThrough (items() As String)
    Dim i As Long
    For i = LBound(items) To UBound(items)
        items(i) = UCase$(items(i))
    Next
End Sub


'$Include:'../utilities/assert.bm'
