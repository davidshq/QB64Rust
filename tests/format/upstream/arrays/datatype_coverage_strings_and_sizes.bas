$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type DataTypesRecord
    Prefix As Long
    Fixed(1 TO 2) As String * 4
    Var(-1 TO 0) As String
    Sng(0 TO 1) As Single
    Dbl(0 TO 1) As Double
    I64(0 TO 1) As _Integer64
    U64(0 TO 1) As _Unsigned _Integer64
    Tail As Long
End Type

Dim x(0) As DataTypesRecord

x(0).Prefix = 10
x(0).Fixed(1) = "AB"
x(0).Fixed(2) = "WXYZ"
x(0).Var(-1) = "hello"
x(0).Var(0) = "QB64PE"
x(0).Sng(0) = 1.5
x(0).Sng(1) = 2.5
x(0).Dbl(0) = 10.25
x(0).Dbl(1) = 20.75
x(0).I64(0) = 1000000000
x(0).I64(1) = -2000000000
x(0).U64(0) = 3000000000
x(0).U64(1) = 4000000000
x(0).Tail = 20

CheckTrue "fixed bounds", LBound(x(0).Fixed) = 1 And UBound(x(0).Fixed) = 2
CheckTrue "var bounds", LBound(x(0).Var) = -1 And UBound(x(0).Var) = 0
CheckTrue "fixed len", Len(x(0).Fixed(1)) = 4 And Len(x(0).Fixed(2)) = 4
CheckText "fixed text 1", RTrim$(x(0).Fixed(1)), "AB"
CheckText "fixed text 2", RTrim$(x(0).Fixed(2)), "WXYZ"
CheckText "var text 1", x(0).Var(-1), "hello"
CheckText "var text 2", x(0).Var(0), "QB64PE"
CheckNear "single values", x(0).Sng(0) + x(0).Sng(1), 4#, .000001
CheckNear "double values", x(0).Dbl(0) + x(0).Dbl(1), 31#, .000001
CheckTrue "int64 values", x(0).I64(0) = 1000000000 And x(0).I64(1) = -2000000000
CheckTrue "uint64 values", x(0).U64(0) = 3000000000 And x(0).U64(1) = 4000000000

Erase x(0).Fixed
CheckTrue "erase fixed len 1", Len(x(0).Fixed(1)) = 4
CheckTrue "erase fixed len 2", Len(x(0).Fixed(2)) = 4
CheckTrue "erase fixed 1 bytes", Asc(x(0).Fixed(1), 1) = 0 And Asc(x(0).Fixed(1), 2) = 0 And Asc(x(0).Fixed(1), 3) = 0 And Asc(x(0).Fixed(1), 4) = 0
CheckTrue "erase fixed 2 bytes", Asc(x(0).Fixed(2), 1) = 0 And Asc(x(0).Fixed(2), 2) = 0 And Asc(x(0).Fixed(2), 3) = 0 And Asc(x(0).Fixed(2), 4) = 0
CheckText "erase fixed keeps var", x(0).Var(-1), "hello"
CheckTrue "erase fixed keeps tail", x(0).Tail = 20

Erase x(0).Var
CheckText "erase var 1", x(0).Var(-1), ""
CheckText "erase var 2", x(0).Var(0), ""
CheckTrue "erase var keeps fixed len", Len(x(0).Fixed(1)) = 4 And Len(x(0).Fixed(2)) = 4
CheckTrue "erase var keeps fixed bytes", Asc(x(0).Fixed(1), 1) = 0 And Asc(x(0).Fixed(1), 2) = 0 And Asc(x(0).Fixed(1), 3) = 0 And Asc(x(0).Fixed(1), 4) = 0 And Asc(x(0).Fixed(2), 1) = 0 And Asc(x(0).Fixed(2), 2) = 0 And Asc(x(0).Fixed(2), 3) = 0 And Asc(x(0).Fixed(2), 4) = 0
CheckTrue "erase var keeps prefix/tail", x(0).Prefix = 10 And x(0).Tail = 20

FinishTest
Rem sleep
System

Sub CheckTrue (testName As String, condition As Long)
    If condition Then
        PassCount = PassCount + 1
    Else
        FailCount = FailCount + 1
        Print "FAIL:"; testName
    End If
End Sub

Sub CheckText (testName As String, actual As String, expected As String)
    If actual = expected Then
        PassCount = PassCount + 1
    Else
        FailCount = FailCount + 1
        Print "FAIL:"; testName; " expected=["; expected; "] actual=["; actual; "]"
    End If
End Sub

Sub CheckNear (testName As String, actual As Double, expected As Double, tolerance As Double)
    If Abs(actual - expected) <= tolerance Then
        PassCount = PassCount + 1
    Else
        FailCount = FailCount + 1
        Print "FAIL:"; testName; " expected="; expected; " actual="; actual
    End If
End Sub

Sub FinishTest
    If FailCount = 0 Then
        Print "RESULT: PASS"
    Else
        Print "RESULT: FAIL"; FailCount; "failure(s)"
    End If
End Sub

