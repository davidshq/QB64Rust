$Console:Only
$Unstable:TypeFields
DefLng A-Z

Type StaticOnly
    recordId As Long
    numbers(0 To 3) _Static As Long
    labels(1 To 2) _Static As String * 8
End Type

Type DynamicOnly
    numbers(0 To 1) _Dynamic As Long
    texts(0 To 1) _Dynamic As String
End Type

Type MixedFields
    recordId As Long
    fixedNumbers(0 To 2) _Static As Long
    fixedNames(0 To 1) _Static As String * 10
    dynNumbers(0 To 1) _Dynamic As Single
    dynTexts(0 To 1) _Dynamic As String
End Type

Type NestedLeaf
    leafId As Long
    fixedData(0 To 1) _Static As Integer
    dynData(0 To 1) _Dynamic As Long
    noteText As String
End Type

Type NestedRoot
    rootId As Long
    leaf As NestedLeaf
    weight As Double
End Type

Type FixedLeaf
    leafId As Long
    samples(0 To 2) _Static As Long
    tagText As String * 8
End Type

Type FixedBranch
    branchId As Long
    leafSet(0 To 2) _Static As FixedLeaf
End Type

Type DynamicLeaf
    leafId As Long
    samples(0 To 1) _Dynamic As Long
    noteText As String
End Type

Type DynamicBranch
    branchId As Long
    leafSet(0 To 1) _Dynamic As DynamicLeaf
End Type

Run008
Run009
Run010
Run011
Run012
Run013

Print "ALL PASS"
System 0

Sub Run008
    Dim srcData(0 To 2) As StaticOnly
    Dim dstData(0 To 2) As StaticOnly
    Dim slotIndex As Long
    Dim itemIndex As Long

    For slotIndex = 0 To 2
        srcData(slotIndex).recordId = 1000 + slotIndex
        For itemIndex = 0 To 3
            srcData(slotIndex).numbers(itemIndex) = slotIndex * 100 + itemIndex
        Next itemIndex
        srcData(slotIndex).labels(1) = "A" + LTrim$(Str$(slotIndex))
        srcData(slotIndex).labels(2) = "B" + LTrim$(Str$(slotIndex))
    Next slotIndex

    dstData() = srcData()

    srcData(1).recordId = -1
    srcData(1).numbers(2) = -2
    srcData(1).labels(2) = "changed"

    For slotIndex = 0 To 2
        If dstData(slotIndex).recordId <> 1000 + slotIndex Then Print "FAIL: t008 scalar"; slotIndex: System 1
        For itemIndex = 0 To 3
            If dstData(slotIndex).numbers(itemIndex) <> slotIndex * 100 + itemIndex Then Print "FAIL: t008 numbers"; slotIndex; itemIndex: System 1
        Next itemIndex
        If RTrim$(dstData(slotIndex).labels(1)) <> "A" + LTrim$(Str$(slotIndex)) Then Print "FAIL: t008 label 1"; slotIndex: System 1
        If RTrim$(dstData(slotIndex).labels(2)) <> "B" + LTrim$(Str$(slotIndex)) Then Print "FAIL: t008 label 2"; slotIndex: System 1
    Next slotIndex
End Sub

Sub Run009
    ReDim srcData(0 To 2) As DynamicOnly
    ReDim dstData(0 To 2) As DynamicOnly

    ReDim srcData(0).numbers(-1 To 1)
    ReDim srcData(0).texts(2 To 3)
    srcData(0).numbers(-1) = 11
    srcData(0).numbers(0) = 12
    srcData(0).numbers(1) = 13
    srcData(0).texts(2) = "zero-two"
    srcData(0).texts(3) = "zero-three"

    ReDim srcData(1).numbers(5 To 7)
    ReDim srcData(1).texts(-2 To 0)
    srcData(1).numbers(5) = 21
    srcData(1).numbers(6) = 22
    srcData(1).numbers(7) = 23
    srcData(1).texts(-2) = "one-minus-two"
    srcData(1).texts(-1) = "one-minus-one"
    srcData(1).texts(0) = "one-zero"

    ReDim srcData(2).numbers(10 To 11)
    ReDim srcData(2).texts(4 To 4)
    srcData(2).numbers(10) = 31
    srcData(2).numbers(11) = 32
    srcData(2).texts(4) = "two-four"

    ReDim dstData(0).numbers(20 To 22)
    ReDim dstData(0).texts(20 To 21)
    dstData(0).texts(20) = "old destination"
    ReDim dstData(1).numbers(30 To 30)
    ReDim dstData(1).texts(30 To 30)
    ReDim dstData(2).numbers(40 To 43)
    ReDim dstData(2).texts(40 To 42)

    dstData() = srcData()

    If LBound(dstData(0).numbers) <> -1 Or UBound(dstData(0).numbers) <> 1 Then Print "FAIL: t009 bounds numbers 0": System 1
    If LBound(dstData(0).texts) <> 2 Or UBound(dstData(0).texts) <> 3 Then Print "FAIL: t009 bounds texts 0": System 1
    If LBound(dstData(1).numbers) <> 5 Or UBound(dstData(1).numbers) <> 7 Then Print "FAIL: t009 bounds numbers 1": System 1
    If LBound(dstData(1).texts) <> -2 Or UBound(dstData(1).texts) <> 0 Then Print "FAIL: t009 bounds texts 1": System 1
    If LBound(dstData(2).numbers) <> 10 Or UBound(dstData(2).numbers) <> 11 Then Print "FAIL: t009 bounds numbers 2": System 1
    If LBound(dstData(2).texts) <> 4 Or UBound(dstData(2).texts) <> 4 Then Print "FAIL: t009 bounds texts 2": System 1

    If dstData(0).numbers(-1) <> 11 Or dstData(0).numbers(1) <> 13 Then Print "FAIL: t009 data 0": System 1
    If dstData(1).numbers(5) <> 21 Or dstData(1).numbers(7) <> 23 Then Print "FAIL: t009 data 1": System 1
    If dstData(2).numbers(10) <> 31 Or dstData(2).numbers(11) <> 32 Then Print "FAIL: t009 data 2": System 1
    If dstData(0).texts(2) <> "zero-two" Or dstData(0).texts(3) <> "zero-three" Then Print "FAIL: t009 text 0": System 1
    If dstData(1).texts(-2) <> "one-minus-two" Or dstData(1).texts(0) <> "one-zero" Then Print "FAIL: t009 text 1": System 1
    If dstData(2).texts(4) <> "two-four" Then Print "FAIL: t009 text 2": System 1

    srcData(0).numbers(-1) = 999
    srcData(1).texts(0) = "changed"
    ReDim srcData(2).numbers(0 To 0)
    srcData(2).numbers(0) = 777

    If dstData(0).numbers(-1) <> 11 Then Print "FAIL: t009 numeric deep copy": System 1
    If dstData(1).texts(0) <> "one-zero" Then Print "FAIL: t009 string deep copy": System 1
    If LBound(dstData(2).numbers) <> 10 Or UBound(dstData(2).numbers) <> 11 Then Print "FAIL: t009 descriptor deep copy": System 1
    If dstData(2).numbers(10) <> 31 Or dstData(2).numbers(11) <> 32 Then Print "FAIL: t009 descriptor data": System 1
End Sub

Sub Run010
    Dim srcData(0 To 1) As MixedFields
    Dim dstData(0 To 1) As MixedFields
    Dim slotIndex As Long
    Dim itemIndex As Long

    For slotIndex = 0 To 1
        srcData(slotIndex).recordId = 500 + slotIndex
        For itemIndex = 0 To 2
            srcData(slotIndex).fixedNumbers(itemIndex) = slotIndex * 10 + itemIndex
        Next itemIndex
        srcData(slotIndex).fixedNames(0) = "fixed-a" + LTrim$(Str$(slotIndex))
        srcData(slotIndex).fixedNames(1) = "fixed-b" + LTrim$(Str$(slotIndex))
    Next slotIndex

    ReDim srcData(0).dynNumbers(-2 To 0)
    ReDim srcData(0).dynTexts(3 To 4)
    srcData(0).dynNumbers(-2) = 1.25
    srcData(0).dynNumbers(-1) = 2.5
    srcData(0).dynNumbers(0) = 3.75
    srcData(0).dynTexts(3) = "dynamic zero a"
    srcData(0).dynTexts(4) = "dynamic zero b"

    ReDim srcData(1).dynNumbers(7 To 8)
    ReDim srcData(1).dynTexts(-1 To 1)
    srcData(1).dynNumbers(7) = 7.5
    srcData(1).dynNumbers(8) = 8.5
    srcData(1).dynTexts(-1) = "dynamic one a"
    srcData(1).dynTexts(0) = "dynamic one b"
    srcData(1).dynTexts(1) = "dynamic one c"

    ReDim dstData(0).dynNumbers(20 To 20)
    ReDim dstData(0).dynTexts(20 To 20)
    ReDim dstData(1).dynNumbers(30 To 32)
    ReDim dstData(1).dynTexts(30 To 31)

    dstData() = srcData()

    For slotIndex = 0 To 1
        If dstData(slotIndex).recordId <> 500 + slotIndex Then Print "FAIL: t010 scalar"; slotIndex: System 1
        For itemIndex = 0 To 2
            If dstData(slotIndex).fixedNumbers(itemIndex) <> slotIndex * 10 + itemIndex Then Print "FAIL: t010 static numbers"; slotIndex; itemIndex: System 1
        Next itemIndex
        If RTrim$(dstData(slotIndex).fixedNames(0)) <> "fixed-a" + LTrim$(Str$(slotIndex)) Then Print "FAIL: t010 static name 0"; slotIndex: System 1
        If RTrim$(dstData(slotIndex).fixedNames(1)) <> "fixed-b" + LTrim$(Str$(slotIndex)) Then Print "FAIL: t010 static name 1"; slotIndex: System 1
    Next slotIndex

    If LBound(dstData(0).dynNumbers) <> -2 Or UBound(dstData(0).dynNumbers) <> 0 Then Print "FAIL: t010 dynamic bounds 0": System 1
    If LBound(dstData(1).dynNumbers) <> 7 Or UBound(dstData(1).dynNumbers) <> 8 Then Print "FAIL: t010 dynamic bounds 1": System 1
    If dstData(0).dynNumbers(-1) <> 2.5 Then Print "FAIL: t010 dynamic number 0": System 1
    If dstData(1).dynNumbers(8) <> 8.5 Then Print "FAIL: t010 dynamic number 1": System 1
    If dstData(0).dynTexts(4) <> "dynamic zero b" Then Print "FAIL: t010 dynamic text 0": System 1
    If dstData(1).dynTexts(1) <> "dynamic one c" Then Print "FAIL: t010 dynamic text 1": System 1

    srcData(0).fixedNumbers(1) = -100
    srcData(0).dynNumbers(-1) = -200
    srcData(1).dynTexts(1) = "changed"
    ReDim srcData(1).dynNumbers(0 To 0)

    If dstData(0).fixedNumbers(1) <> 1 Then Print "FAIL: t010 static independence": System 1
    If dstData(0).dynNumbers(-1) <> 2.5 Then Print "FAIL: t010 dynamic independence": System 1
    If dstData(1).dynTexts(1) <> "dynamic one c" Then Print "FAIL: t010 string independence": System 1
    If LBound(dstData(1).dynNumbers) <> 7 Or UBound(dstData(1).dynNumbers) <> 8 Then Print "FAIL: t010 descriptor independence": System 1
End Sub

Sub Run011
    ReDim srcData(0 To 1) As NestedRoot
    ReDim dstData(0 To 1) As NestedRoot

    srcData(0).rootId = 100
    srcData(0).weight = 1.5
    srcData(0).leaf.leafId = 101
    srcData(0).leaf.fixedData(0) = 11
    srcData(0).leaf.fixedData(1) = 12
    srcData(0).leaf.noteText = "root zero leaf"
    ReDim srcData(0).leaf.dynData(-1 To 1)
    srcData(0).leaf.dynData(-1) = 13
    srcData(0).leaf.dynData(0) = 14
    srcData(0).leaf.dynData(1) = 15

    srcData(1).rootId = 200
    srcData(1).weight = 2.5
    srcData(1).leaf.leafId = 201
    srcData(1).leaf.fixedData(0) = 21
    srcData(1).leaf.fixedData(1) = 22
    srcData(1).leaf.noteText = "root one leaf"
    ReDim srcData(1).leaf.dynData(5 To 6)
    srcData(1).leaf.dynData(5) = 23
    srcData(1).leaf.dynData(6) = 24

    ReDim dstData(0).leaf.dynData(20 To 21)
    dstData(0).leaf.noteText = "old zero"
    ReDim dstData(1).leaf.dynData(30 To 30)
    dstData(1).leaf.noteText = "old one"

    dstData() = srcData()

    If dstData(0).rootId <> 100 Or dstData(0).leaf.leafId <> 101 Then Print "FAIL: t011 scalar 0": System 1
    If dstData(1).rootId <> 200 Or dstData(1).leaf.leafId <> 201 Then Print "FAIL: t011 scalar 1": System 1
    If dstData(0).leaf.fixedData(0) <> 11 Or dstData(0).leaf.fixedData(1) <> 12 Then Print "FAIL: t011 fixed 0": System 1
    If dstData(1).leaf.fixedData(0) <> 21 Or dstData(1).leaf.fixedData(1) <> 22 Then Print "FAIL: t011 fixed 1": System 1
    If dstData(0).leaf.noteText <> "root zero leaf" Then Print "FAIL: t011 text 0": System 1
    If dstData(1).leaf.noteText <> "root one leaf" Then Print "FAIL: t011 text 1": System 1
    If LBound(dstData(0).leaf.dynData) <> -1 Or UBound(dstData(0).leaf.dynData) <> 1 Then Print "FAIL: t011 bounds 0": System 1
    If LBound(dstData(1).leaf.dynData) <> 5 Or UBound(dstData(1).leaf.dynData) <> 6 Then Print "FAIL: t011 bounds 1": System 1
    If dstData(0).leaf.dynData(0) <> 14 Or dstData(1).leaf.dynData(6) <> 24 Then Print "FAIL: t011 dynamic data": System 1

    srcData(0).leaf.fixedData(0) = -1
    srcData(0).leaf.noteText = "changed"
    srcData(0).leaf.dynData(0) = -2
    ReDim srcData(1).leaf.dynData(0 To 0)

    If dstData(0).leaf.fixedData(0) <> 11 Then Print "FAIL: t011 fixed independence": System 1
    If dstData(0).leaf.noteText <> "root zero leaf" Then Print "FAIL: t011 text independence": System 1
    If dstData(0).leaf.dynData(0) <> 14 Then Print "FAIL: t011 dynamic independence": System 1
    If LBound(dstData(1).leaf.dynData) <> 5 Or UBound(dstData(1).leaf.dynData) <> 6 Then Print "FAIL: t011 nested descriptor independence": System 1
End Sub

Sub Run012
    Dim srcData(0 To 1) As FixedBranch
    Dim dstData(0 To 1) As FixedBranch
    Dim branchIndex As Long
    Dim leafIndex As Long
    Dim sampleIndex As Long

    For branchIndex = 0 To 1
        srcData(branchIndex).branchId = 1000 + branchIndex
        For leafIndex = 0 To 2
            srcData(branchIndex).leafSet(leafIndex).leafId = branchIndex * 100 + leafIndex
            srcData(branchIndex).leafSet(leafIndex).tagText = "T" + LTrim$(Str$(branchIndex)) + LTrim$(Str$(leafIndex))
            For sampleIndex = 0 To 2
                srcData(branchIndex).leafSet(leafIndex).samples(sampleIndex) = branchIndex * 1000 + leafIndex * 10 + sampleIndex
            Next sampleIndex
        Next leafIndex
    Next branchIndex

    dstData() = srcData()

    srcData(1).leafSet(2).leafId = -1
    srcData(1).leafSet(2).samples(1) = -2
    srcData(1).leafSet(2).tagText = "changed"

    For branchIndex = 0 To 1
        If dstData(branchIndex).branchId <> 1000 + branchIndex Then Print "FAIL: t012 branch"; branchIndex: System 1
        For leafIndex = 0 To 2
            If dstData(branchIndex).leafSet(leafIndex).leafId <> branchIndex * 100 + leafIndex Then Print "FAIL: t012 leaf"; branchIndex; leafIndex: System 1
            If RTrim$(dstData(branchIndex).leafSet(leafIndex).tagText) <> "T" + LTrim$(Str$(branchIndex)) + LTrim$(Str$(leafIndex)) Then Print "FAIL: t012 tag"; branchIndex; leafIndex: System 1
            For sampleIndex = 0 To 2
                If dstData(branchIndex).leafSet(leafIndex).samples(sampleIndex) <> branchIndex * 1000 + leafIndex * 10 + sampleIndex Then Print "FAIL: t012 sample"; branchIndex; leafIndex; sampleIndex: System 1
            Next sampleIndex
        Next leafIndex
    Next branchIndex
End Sub

Sub Run013
    ReDim srcData(0 To 1) As DynamicBranch
    ReDim dstData(0 To 1) As DynamicBranch

    srcData(0).branchId = 100
    ReDim srcData(0).leafSet(1 To 2)
    srcData(0).leafSet(1).leafId = 101
    srcData(0).leafSet(1).noteText = "zero leaf one"
    ReDim srcData(0).leafSet(1).samples(-1 To 0)
    srcData(0).leafSet(1).samples(-1) = 11
    srcData(0).leafSet(1).samples(0) = 12
    srcData(0).leafSet(2).leafId = 102
    srcData(0).leafSet(2).noteText = "zero leaf two"
    ReDim srcData(0).leafSet(2).samples(5 To 7)
    srcData(0).leafSet(2).samples(5) = 13
    srcData(0).leafSet(2).samples(6) = 14
    srcData(0).leafSet(2).samples(7) = 15

    srcData(1).branchId = 200
    ReDim srcData(1).leafSet(-1 To 0)
    srcData(1).leafSet(-1).leafId = 201
    srcData(1).leafSet(-1).noteText = "one leaf minus one"
    ReDim srcData(1).leafSet(-1).samples(8 To 8)
    srcData(1).leafSet(-1).samples(8) = 21
    srcData(1).leafSet(0).leafId = 202
    srcData(1).leafSet(0).noteText = "one leaf zero"
    ReDim srcData(1).leafSet(0).samples(9 To 10)
    srcData(1).leafSet(0).samples(9) = 22
    srcData(1).leafSet(0).samples(10) = 23

    ReDim dstData(0).leafSet(20 To 20)
    dstData(0).leafSet(20).noteText = "old destination zero"
    ReDim dstData(0).leafSet(20).samples(20 To 21)
    ReDim dstData(1).leafSet(30 To 31)
    dstData(1).leafSet(30).noteText = "old destination one"
    ReDim dstData(1).leafSet(30).samples(30 To 30)

    dstData() = srcData()

    If dstData(0).branchId <> 100 Or dstData(1).branchId <> 200 Then Print "FAIL: t013 branch scalar": System 1
    If LBound(dstData(0).leafSet) <> 1 Or UBound(dstData(0).leafSet) <> 2 Then Print "FAIL: t013 leaf bounds 0": System 1
    If LBound(dstData(1).leafSet) <> -1 Or UBound(dstData(1).leafSet) <> 0 Then Print "FAIL: t013 leaf bounds 1": System 1
    If dstData(0).leafSet(1).leafId <> 101 Or dstData(0).leafSet(2).leafId <> 102 Then Print "FAIL: t013 leaf ids 0": System 1
    If dstData(1).leafSet(-1).leafId <> 201 Or dstData(1).leafSet(0).leafId <> 202 Then Print "FAIL: t013 leaf ids 1": System 1
    If dstData(0).leafSet(1).noteText <> "zero leaf one" Then Print "FAIL: t013 note 0-1": System 1
    If dstData(0).leafSet(2).noteText <> "zero leaf two" Then Print "FAIL: t013 note 0-2": System 1
    If dstData(1).leafSet(-1).noteText <> "one leaf minus one" Then Print "FAIL: t013 note 1--1": System 1
    If dstData(1).leafSet(0).noteText <> "one leaf zero" Then Print "FAIL: t013 note 1-0": System 1
    If LBound(dstData(0).leafSet(1).samples) <> -1 Or UBound(dstData(0).leafSet(1).samples) <> 0 Then Print "FAIL: t013 sample bounds 0-1": System 1
    If LBound(dstData(0).leafSet(2).samples) <> 5 Or UBound(dstData(0).leafSet(2).samples) <> 7 Then Print "FAIL: t013 sample bounds 0-2": System 1
    If LBound(dstData(1).leafSet(-1).samples) <> 8 Or UBound(dstData(1).leafSet(-1).samples) <> 8 Then Print "FAIL: t013 sample bounds 1--1": System 1
    If LBound(dstData(1).leafSet(0).samples) <> 9 Or UBound(dstData(1).leafSet(0).samples) <> 10 Then Print "FAIL: t013 sample bounds 1-0": System 1
    If dstData(0).leafSet(1).samples(-1) <> 11 Or dstData(0).leafSet(2).samples(7) <> 15 Then Print "FAIL: t013 samples 0": System 1
    If dstData(1).leafSet(-1).samples(8) <> 21 Or dstData(1).leafSet(0).samples(10) <> 23 Then Print "FAIL: t013 samples 1": System 1

    srcData(0).leafSet(1).noteText = "changed"
    srcData(0).leafSet(2).samples(7) = 999
    ReDim srcData(1).leafSet(0).samples(0 To 0)
    ReDim srcData(0).leafSet(0 To 0)

    If dstData(0).leafSet(1).noteText <> "zero leaf one" Then Print "FAIL: t013 nested string independence": System 1
    If dstData(0).leafSet(2).samples(7) <> 15 Then Print "FAIL: t013 nested data independence": System 1
    If LBound(dstData(1).leafSet(0).samples) <> 9 Or UBound(dstData(1).leafSet(0).samples) <> 10 Then Print "FAIL: t013 nested descriptor independence": System 1
    If LBound(dstData(0).leafSet) <> 1 Or UBound(dstData(0).leafSet) <> 2 Then Print "FAIL: t013 outer descriptor independence": System 1
End Sub
