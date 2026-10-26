$Console:Only

Type TestRec
    n As String
    m As Long
    l As String * 8
End Type

Dim f As Long
Dim loopA As Long
Dim loopB As Long
Dim loopC As Long

Dim NL As String
Dim Actual As String
Dim Expected As String

Dim ExpectedDynamic As String
Dim ExpectedDynamic3D As String
Dim ExpectedFixed1 As String * 8
Dim ExpectedFixed2 As String * 8
Dim ExpectedM As Long
Dim ExpectedM3D As Long
Dim ExpectedPreserveValue As Long

NL = Chr$(10)

ExpectedDynamic = "dynamic"
ExpectedDynamic3D = "3D test dynamic"
ExpectedFixed1 = "Qbasic"
ExpectedFixed2 = "fixed"
ExpectedM = 120000
ExpectedM3D = 199201
ExpectedPreserveValue = 123

Actual = ""
Expected = ""

ReDim T(5) As TestRec

T(1).n = "dynamic"
T(1).m = 120000
T(1).l = "Qbasic"

For f = 1 To 1000
    ReDim _Preserve T(5 + f) As TestRec
Next f

Actual = Actual + T(1).n + NL
Expected = Expected + ExpectedDynamic + NL

Actual = Actual + Str$(T(1).m) + NL
Expected = Expected + Str$(ExpectedM) + NL

Actual = Actual + T(1).l + NL
Expected = Expected + ExpectedFixed1 + NL

Actual = Actual + Str$(UBound(T)) + NL
Expected = Expected + Str$(1005) + NL


ReDim _Retain T(5) As TestRec

Actual = Actual + T(1).n + NL
Expected = Expected + ExpectedDynamic + NL

Actual = Actual + Str$(T(1).m) + NL
Expected = Expected + Str$(ExpectedM) + NL

Actual = Actual + T(1).l + NL
Expected = Expected + ExpectedFixed1 + NL

Actual = Actual + Str$(UBound(T)) + NL
Expected = Expected + Str$(5) + NL


For f = 1 To 1000
    ReDim _Retain T(5 + f) As TestRec
Next f

Actual = Actual + T(1).n + NL
Expected = Expected + ExpectedDynamic + NL

Actual = Actual + Str$(T(1).m) + NL
Expected = Expected + Str$(ExpectedM) + NL

Actual = Actual + T(1).l + NL
Expected = Expected + ExpectedFixed1 + NL

Actual = Actual + Str$(UBound(T)) + NL
Expected = Expected + Str$(1005) + NL


ReDim U(10, 100, 10) As TestRec

U(1, 2, 3).n = "3D test dynamic"
U(6, 5, 4).l = "fixed"
U(6, 2, 4).m = 199201

For loopA = 10 To 6 Step -1
    For loopB = 5 To 6
        For loopC = 7 To 4 Step -1
            ReDim _Retain U(loopA, loopB, loopC) As TestRec
        Next loopC
    Next loopB
Next loopA

Actual = Actual + U(1, 2, 3).n + NL
Expected = Expected + ExpectedDynamic3D + NL

Actual = Actual + U(6, 5, 4).l + NL
Expected = Expected + ExpectedFixed2 + NL

Actual = Actual + Str$(U(6, 2, 4).m) + NL
Expected = Expected + Str$(ExpectedM3D) + NL


' Preserve logic

ReDim PreserveCheck(5 To 10) As Long
PreserveCheck(5) = 123

ReDim _Preserve PreserveCheck(20 To 40) As Long

Actual = Actual + Str$(PreserveCheck(20)) + NL
Expected = Expected + Str$(ExpectedPreserveValue) + NL


If Actual = Expected Then
    Print "PASS"
Else
    Print "FAIL"
End If

System
