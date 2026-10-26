$Console:Only
_Define A-Z As LONG 
Option _Explicit

Dim As Long x

Print 1 Or 2
Print 3 And 1
Print Not 2
Print 2 Xor 3
Print 10 Eqv 5
Print 20 Imp 50

Print 20 Mod 3
Print 2 + 3
Print 2 - 3
Print 6 / 3
Print 7 \ 3
Print 7 * 20
Print 3 ^ 10

x = 20
Print -x

Print 2 = 2
Print 2 <> 3
Print 2 <> 3
Print 2 <= 3
Print 2 <= 3
Print 2 >= 3
Print 2 >= 3
Print 2 > 3
Print 2 < 3

' The left side which has no parens to indicate order should still equal the
' right side which has parens to enforce order.
Print (2 ^ 2 * 2) = ((2 ^ 2) * 2)
Print (2 ^ 2 + 2) = ((2 ^ 2) + 2)
Print (Not 2 + 3) = (Not (2 + 3))
Print (-2 ^ 2) = (-(2 ^ 2))
Print (Not 2 ^ 3) = (Not (2 ^ 3))
x = 0
Print 2 ^ -Not x
x = 1
Print 2 ^ -Not x
x = 0
Print 2 ^ -Not x + 3
Print (3 * 6 / 2) = ((3 * 6) / 2)
Print (3 * 10 \ 3) = ((3 * 10) \ 3)

' Many levels of parens
Print (2 ^ (3 * (4 - (2 - (10 / (20 / 2))))))

Const foo = "foo"
Const bar = "bar"
Print foo + bar

' Combos
x = 1
Print Not (x = 0) And (x <> -1)
Print Not (x = 0) _AndAlso (x <> -1)
Print _Negate (x = 0) And (x <> -1)
Print _Negate (x = 0) _AndAlso (x <> -1)

x = 0
Print (3 * 2) + (4 - 2) / (5 ^ 2) < 5 And Not (x = 0)
Print (3 * 2) + (4 - 2) / (5 ^ 2) < 5 And _Negate (x = 0)

Const A = -100
Const B = -10
Const D = 10
Const E = 100

' More NOT & _NEGATE tests
x = -1
Print Not A
Print Not D
Print Not x
x = 0
Print Not x
x = 1
Print Not x

x = -1
Print _Negate A
Print _Negate D
Print _Negate x
x = 0
Print _Negate x
x = 1
Print _Negate x

x = 1000
Print Not x > 2000 Or E = GetValue(123456)
Print Not x > 2000 And E <> GetValue(123456)

Print Not x > 2000 _OrElse E = GetValue(123456)
Print Not x > 2000 _AndAlso E <> GetValue(123456)

x = 0
x = _Negate (_Negate (_Negate (_Negate x)))

If _Negate GetValue(x) Then
    Print "_NEGATE: Test passed."
Else
    Print "_NEGATE: Test failed."
End If

If GetValue(D) < 0 _AndAlso GetValue(E) < 0 Then
    Print "_ANDALSO: Test failed."
Else
    Print "_ANDALSO: Test passed."
End If

If GetValue(A) < 0 _OrElse GetValue(B) < 0 Then
    Print "_ORELSE: Test passed."
Else
    Print "_ORELSE: Test failed."
End If

x = 0
x = Not (Not (Not (Not x)))

If Not GetValue(x) Then
    Print "NOT: Test passed."
Else
    Print "NOT: Test failed."
End If

If GetValue(D) < 0 And GetValue(E) < 0 Then
    Print "AND: Test failed."
Else
    Print "AND: Test passed."
End If

If GetValue(A) < 0 Or GetValue(B) < 0 Then
    Print "OR: Test passed."
Else
    Print "OR: Test failed."
End If

System

Function GetValue& (x As Long)
    Print "Function called for value:"; x
    GetValue = x
End Function
