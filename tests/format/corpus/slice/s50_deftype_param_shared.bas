$Console:Only
' A plain name in a procedure that a DIM SHARED ... AS types in the main module (spec language/default-types;
' measured, v24_a_param_shared): it means the procedure's own variable of that name and the default type (a
' parameter, a local), else the shared one: after a DEFxxx in the body, for another type, for one bound by SHARED.
Dim Shared x As Long
Dim Shared y As String
Dim Shared k As Double
Dim Shared m As Long
x = 7: y = "main": k = 1.5: m = 70
s 1.5, 2.5
Print "1:"; x; y
Print "2:"; f(1.5)
Print "3:"; x
t 2.5
Print "4:"; m
u 2.5
Print "5:"; k
w 1
Print "6:"; x
x! = 1.25
h 1
Print "7:"; x; x!
p 1.5, 2.5
Print "8:"; x
System

Sub s (x, y)
    Print "s:"; x; y
    x = 3.25
End Sub

Function f (x)
    f = x * 2
    x = 9
End Function

' The default changes in the body: what the plain name means below the DEFINT.
Sub t (m)
    Print "t1:"; m
    DefInt A-Z
    Print "t2:"; m
    m = 9
    Print "t3:"; m
End Sub

' Below the DEFINT of t: the parameter is an INTEGER.
Sub u (k)
    Print "u:"; k
    k = 4
    Print "u:"; k
End Sub

' Still below the DEFINT of t (the default is INTEGER). A local variable written with the default type's suffix:
' the plain name means it, not the shared one.
Sub w (q)
    x% = 3
    Print "w:"; x
    x = 2
    Print "w:"; x%
End Sub

' A main-module variable bound by SHARED with the default type's suffix: the plain name stays the DIM SHARED one.
DefSng A-Z
Sub h (q)
    Shared x!
    Print "h:"; x
    x = 5
End Sub

' Parameters with a suffix: the default type's (the parameter) and another (the shared variable).
Sub p (x!, m%)
    Print "p:"; x; m
    x = 6
End Sub
