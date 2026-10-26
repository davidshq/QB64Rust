$Console:Only
$Unstable:TypeFields
DefLng A-Z

Type NegLeaf
    valueId As Long
    samples(0 To 1) _Dynamic As Long
End Type

Type NegDynBranch
    leaves(0 To 1) _Dynamic As NegLeaf
End Type

Type NegStaticBranch
    leaves(-2 To -1) _Static As NegLeaf
End Type

Type StaticBox
    values(0 To 2) _Static As Long
    tags(0 To 1) _Static As String * 8
End Type

Type DynamicBox
    values(0 To 1) _Dynamic As Long
    words(0 To 1) _Dynamic As String
End Type

Type PlainLeaf
    leafId As Long
    score As Single
End Type

Type OwnerLeaf
    leafId As Long
    noteText As String
    samples(0 To 1) _Dynamic As Long
End Type

Type BranchData
    fixedLeaf(0 To 1) _Static As PlainLeaf
    ownedLeaf(0 To 1) _Dynamic As OwnerLeaf
End Type

Run014
Run016
Run017
Run018

Print "ALL PASS"
System 0

Sub Run014
    ReDim dynRoot(-1 To 0) As NegDynBranch
    Dim staticRoot As NegStaticBranch

    ' Negative index in the top-level parent array.
    ReDim dynRoot(-1).leaves(-2 To -1)

    ' Negative index in an intermediate descriptor-backed UDT member array.
    ReDim dynRoot(-1).leaves(-2).samples(8 To 9)
    dynRoot(-1).leaves(-2).samples(8) = 81
    dynRoot(-1).leaves(-2).samples(9) = 91

    ' Negative index in an intermediate inline/_Static UDT member array.
    ReDim staticRoot.leaves(-2).samples(3 To 4)
    staticRoot.leaves(-2).samples(3) = 31
    staticRoot.leaves(-2).samples(4) = 41

    If LBound(dynRoot(-1).leaves) <> -2 Or UBound(dynRoot(-1).leaves) <> -1 Then Print "FAIL: t014 dynamic leaf bounds": System 1
    If LBound(dynRoot(-1).leaves(-2).samples) <> 8 Or UBound(dynRoot(-1).leaves(-2).samples) <> 9 Then Print "FAIL: t014 dynamic sample bounds": System 1
    If dynRoot(-1).leaves(-2).samples(8) <> 81 Or dynRoot(-1).leaves(-2).samples(9) <> 91 Then Print "FAIL: t014 dynamic values": System 1
    If LBound(staticRoot.leaves(-2).samples) <> 3 Or UBound(staticRoot.leaves(-2).samples) <> 4 Then Print "FAIL: t014 static sample bounds": System 1
    If staticRoot.leaves(-2).samples(3) <> 31 Or staticRoot.leaves(-2).samples(4) <> 41 Then Print "FAIL: t014 static values": System 1
End Sub

Sub Run016
    Dim srcBox As StaticBox
    Dim dstBox As StaticBox
    Dim normalVals(0 To 2) As Long

    srcBox.values(0) = 11
    srcBox.values(1) = 22
    srcBox.values(2) = 33
    srcBox.tags(0) = "alpha"
    srcBox.tags(1) = "beta"

    dstBox.values() = srcBox.values()
    dstBox.tags() = srcBox.tags()

    If dstBox.values(0) <> 11 Or dstBox.values(1) <> 22 Or dstBox.values(2) <> 33 Then Print "FAIL: t016 member copy": System 1
    If RTrim$(dstBox.tags(0)) <> "alpha" Or RTrim$(dstBox.tags(1)) <> "beta" Then Print "FAIL: t016 fixed strings": System 1

    normalVals() = srcBox.values()
    If normalVals(0) <> 11 Or normalVals(1) <> 22 Or normalVals(2) <> 33 Then Print "FAIL: t016 member to normal": System 1

    normalVals(0) = 71
    normalVals(1) = 72
    normalVals(2) = 73
    dstBox.values() = normalVals()
    If dstBox.values(0) <> 71 Or dstBox.values(1) <> 72 Or dstBox.values(2) <> 73 Then Print "FAIL: t016 normal to member": System 1

    srcBox.values(0) = -1
    If dstBox.values(0) <> 71 Then Print "FAIL: t016 independence": System 1
End Sub

Sub Run017
    Dim srcBox As DynamicBox
    Dim dstBox As DynamicBox
    Dim normalWords(2 To 3) As String

    ReDim srcBox.values(-2 To 0)
    ReDim dstBox.values(-2 To 0)
    ReDim srcBox.words(2 To 3)
    ReDim dstBox.words(2 To 3)

    srcBox.values(-2) = 12
    srcBox.values(-1) = 13
    srcBox.values(0) = 14
    srcBox.words(2) = "dynamic two"
    srcBox.words(3) = "dynamic three"

    dstBox.values() = srcBox.values()
    dstBox.words() = srcBox.words()

    If LBound(dstBox.values) <> -2 Or UBound(dstBox.values) <> 0 Then Print "FAIL: t017 numeric bounds": System 1
    If dstBox.values(-2) <> 12 Or dstBox.values(-1) <> 13 Or dstBox.values(0) <> 14 Then Print "FAIL: t017 numeric payload": System 1
    If dstBox.words(2) <> "dynamic two" Or dstBox.words(3) <> "dynamic three" Then Print "FAIL: t017 string payload": System 1

    normalWords() = srcBox.words()
    If normalWords(2) <> "dynamic two" Or normalWords(3) <> "dynamic three" Then Print "FAIL: t017 member to normal": System 1

    normalWords(2) = "normal two"
    normalWords(3) = "normal three"
    dstBox.words() = normalWords()
    If dstBox.words(2) <> "normal two" Or dstBox.words(3) <> "normal three" Then Print "FAIL: t017 normal to member": System 1

    srcBox.words(2) = "changed source"
    If dstBox.words(2) <> "normal two" Then Print "FAIL: t017 deep copy": System 1
End Sub

Sub Run018
    Dim srcBranch As BranchData
    Dim dstBranch As BranchData

    ReDim srcBranch.ownedLeaf(0 To 1)
    ReDim dstBranch.ownedLeaf(0 To 1)
    ReDim srcBranch.ownedLeaf(0).samples(4 To 5)
    ReDim srcBranch.ownedLeaf(1).samples(7 To 8)
    ReDim dstBranch.ownedLeaf(0).samples(4 To 5)
    ReDim dstBranch.ownedLeaf(1).samples(7 To 8)

    srcBranch.fixedLeaf(0).leafId = 10
    srcBranch.fixedLeaf(0).score = 1.5
    srcBranch.fixedLeaf(1).leafId = 20
    srcBranch.fixedLeaf(1).score = 2.5

    srcBranch.ownedLeaf(0).leafId = 101
    srcBranch.ownedLeaf(0).noteText = "owner zero"
    srcBranch.ownedLeaf(0).samples(4) = 41
    srcBranch.ownedLeaf(0).samples(5) = 42
    srcBranch.ownedLeaf(1).leafId = 102
    srcBranch.ownedLeaf(1).noteText = "owner one"
    srcBranch.ownedLeaf(1).samples(7) = 71
    srcBranch.ownedLeaf(1).samples(8) = 72

    dstBranch.fixedLeaf() = srcBranch.fixedLeaf()
    dstBranch.ownedLeaf() = srcBranch.ownedLeaf()

    If dstBranch.fixedLeaf(0).leafId <> 10 Or dstBranch.fixedLeaf(1).leafId <> 20 Then Print "FAIL: t018 static UDT array": System 1
    If dstBranch.ownedLeaf(0).leafId <> 101 Or dstBranch.ownedLeaf(1).leafId <> 102 Then Print "FAIL: t018 dynamic UDT ids": System 1
    If dstBranch.ownedLeaf(0).noteText <> "owner zero" Or dstBranch.ownedLeaf(1).noteText <> "owner one" Then Print "FAIL: t018 dynamic UDT strings": System 1
    If dstBranch.ownedLeaf(0).samples(4) <> 41 Or dstBranch.ownedLeaf(0).samples(5) <> 42 Then Print "FAIL: t018 nested samples 0": System 1
    If dstBranch.ownedLeaf(1).samples(7) <> 71 Or dstBranch.ownedLeaf(1).samples(8) <> 72 Then Print "FAIL: t018 nested samples 1": System 1

    srcBranch.ownedLeaf(0).noteText = "changed source"
    ReDim _Retain srcBranch.ownedLeaf(0).samples(4 To 6)
    srcBranch.ownedLeaf(0).samples(4) = -1
    If dstBranch.ownedLeaf(0).noteText <> "owner zero" Then Print "FAIL: t018 string independence": System 1
    If UBound(dstBranch.ownedLeaf(0).samples) <> 5 Or dstBranch.ownedLeaf(0).samples(4) <> 41 Then Print "FAIL: t018 descriptor independence": System 1
End Sub
