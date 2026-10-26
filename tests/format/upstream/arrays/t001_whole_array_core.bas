$Console:Only
Option Base 1
DefLng A-Z

Type ParseNum
    typ As Long
End Type

Type PairType
    a As Long
    b As Single
End Type

Type TextItem
    number As Long
    textValue As String
End Type

Type PairData
    code As Long
    amount As Single
End Type

On Error GoTo ErrorHandler

Run001
Run002
Run003
Run004
Run005
Run006
Run007

ErrorHandler:
If Err <> 9 Then Print "FAIL: t007 unexpected error"; Err: System 1
Run015


' Parser/name compatibility cases formerly t037-t039.
ReDim Shared T(1 To 8) As Integer
Dim tempNum As ParseNum
T(1) = 77
tempNum.typ = 1234
t = tempNum.typ
If t <> 1234 Then Print "FAIL: t037 scalar t": System 1
If T(1) <> 77 Then Print "FAIL: t037 array T": System 1

t = T(1)
If t <> 77 Then Print "FAIL: t038 shared array index to scalar": System 1

Dim valueSet(0 To 2) As Long
Dim valueSet As Long
valueSet = 19
valueSet(0) = 83
If valueSet <> 19 Then Print "FAIL: t039 scalar": System 1
If valueSet(0) <> 83 Then Print "FAIL: t039 array": System 1

Print "ALL PASS"
System 0


Sub Run001
    Dim As PairType srcPair(0 To 5), dstPair(0 To 5)
    Dim srcNum(0 To 5) As Long
    Dim dstNum(0 To 5) As Long
    Dim idx As Long

    srcPair(0).a = 1024
    srcPair(1).b = 3.1415

    For idx = 0 To 5
        srcNum(idx) = idx + 1
    Next idx

    dstPair() = srcPair()
    dstNum() = srcNum()

    If dstPair(0).a <> 1024 Then Print "FAIL: t001 fixed UDT index 0": System 1
    If Abs(dstPair(1).b - 3.1415) > .0001 Then Print "FAIL: t001 fixed UDT index 1": System 1

    For idx = 0 To 5
        If dstNum(idx) <> idx + 1 Then Print "FAIL: t001 numeric index"; idx: System 1
    Next idx
End Sub

Sub Run002
    Dim srcText(0 To 2) As String
    Dim dstText(0 To 2) As String
    Dim srcFixed(0 To 2) As String * 8
    Dim dstFixed(0 To 2) As String * 8

    srcText(0) = "zero"
    srcText(1) = "one"
    srcText(2) = "two"

    srcFixed(0) = "A"
    srcFixed(1) = "BBBB"
    srcFixed(2) = "CCCCCCCC"

    dstText() = srcText()
    dstFixed() = srcFixed()

    srcText(1) = "changed"
    srcFixed(1) = "changed"

    If dstText(0) <> "zero" Then Print "FAIL: t002 variable STRING index 0": System 1
    If dstText(1) <> "one" Then Print "FAIL: t002 variable STRING deep copy": System 1
    If dstText(2) <> "two" Then Print "FAIL: t002 variable STRING index 2": System 1
    If dstFixed(0) <> "A" + Space$(7) Then Print "FAIL: t002 fixed STRING index 0": System 1
    If dstFixed(1) <> "BBBB" + Space$(4) Then Print "FAIL: t002 fixed STRING index 1": System 1
    If dstFixed(2) <> "CCCCCCCC" Then Print "FAIL: t002 fixed STRING index 2": System 1
End Sub

Sub Run003
    Dim srcGrid(2, 3) As Long
    Dim dstGrid(2, 3) As Long
    Dim rowIndex As Long
    Dim colIndex As Long

    For rowIndex = 1 To 2
        For colIndex = 1 To 3
            srcGrid(rowIndex, colIndex) = rowIndex * 100 + colIndex
        Next colIndex
    Next rowIndex

    dstGrid() = srcGrid()

    For rowIndex = 1 To 2
        For colIndex = 1 To 3
            If dstGrid(rowIndex, colIndex) <> rowIndex * 100 + colIndex Then
                Print "FAIL: t003 2D at"; rowIndex; colIndex
                System 1
            End If
        Next colIndex
    Next rowIndex
End Sub

Sub Run004
    Dim srcItem(0 To 2) As TextItem
    Dim dstItem(0 To 2) As TextItem

    srcItem(0).number = 10
    srcItem(0).textValue = "first"
    srcItem(1).number = 20
    srcItem(1).textValue = "second"
    srcItem(2).number = 30
    srcItem(2).textValue = "third"

    dstItem() = srcItem()

    srcItem(1).number = 999
    srcItem(1).textValue = "changed"

    If dstItem(0).number <> 10 Or dstItem(0).textValue <> "first" Then Print "FAIL: t004 UDT varstring index 0": System 1
    If dstItem(1).number <> 20 Or dstItem(1).textValue <> "second" Then Print "FAIL: t004 UDT varstring deep copy": System 1
    If dstItem(2).number <> 30 Or dstItem(2).textValue <> "third" Then Print "FAIL: t004 UDT varstring index 2": System 1
End Sub

Sub Run005
    Dim srcValue(0 To 5) As Long
    ReDim dstValue(0 To 5) As Long
    Dim idx As Long

    For idx = 0 To 5
        srcValue(idx) = 1000 + idx
    Next idx

    dstValue() = srcValue()

    For idx = 0 To 5
        If dstValue(idx) <> 1000 + idx Then Print "FAIL: t005 DIM to REDIM index"; idx: System 1
    Next idx
End Sub

Sub Run006
    Dim values(0 To 3) As Long
    Dim idx As Long

    For idx = 0 To 3
        values(idx) = idx * 11
    Next idx

    values() = values()

    For idx = 0 To 3
        If values(idx) <> idx * 11 Then Print "FAIL: t006 self assignment index"; idx: System 1
    Next idx
End Sub

Sub Run007


    Dim srcValue(0 To 5) As Long
    Dim dstValue(1 To 6) As Long

    dstValue() = srcValue()
    Print "FAIL: t007 bounds mismatch was accepted"
    System 1

End Sub

Sub Run015
    Dim srcNum(0 To 5) As Long
    Dim dstNum(0 To 5) As Long
    Dim srcPair(0 To 2) As PairData
    Dim dstPair(0 To 2) As PairData
    Dim idx As Long

    For idx = 0 To 5
        srcNum(idx) = 100 + idx
    Next idx

    dstNum() = srcNum()
    For idx = 0 To 5
        If dstNum(idx) <> 100 + idx Then Print "FAIL: t015 numeric"; idx: System 1
    Next idx

    dstNum(2) = srcNum(4)
    If dstNum(2) <> 104 Then Print "FAIL: t015 indexed": System 1

    For idx = 0 To 2
        srcPair(idx).code = 1000 + idx
        srcPair(idx).amount = idx + .25
    Next idx

    dstPair() = srcPair()
    For idx = 0 To 2
        If dstPair(idx).code <> 1000 + idx Then Print "FAIL: t015 UDT code"; idx: System 1
        If Abs(dstPair(idx).amount - (idx + .25)) > .0001 Then Print "FAIL: t015 UDT amount"; idx: System 1
    Next idx
End Sub
