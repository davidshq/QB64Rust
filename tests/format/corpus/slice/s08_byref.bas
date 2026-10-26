$Console:Only
' Argument passing (spec language/procedures, "Argument passing"): a plain variable of exactly the parameter's type
' is passed by reference; anything else (parentheses, an expression, another type) is converted as for an
' assignment and passed as a copy whose changes are lost.
Dim n As Long
Dim i As Integer
Dim d As Double
n = 1
Call bump(n)
Print "after CALL bump(n):"; n
bump n
Print "after bump n:"; n
bump (n)
Print "after bump (n):"; n
bump n + 1
Print "after bump n + 1:"; n
Call bump((n))
Print "after CALL bump((n)):"; n
x& = 10
bump x&
Print "after bump x&:"; x&
i = 5
bump i
Print "INTEGER after bump i:"; i
d = 2.5
bump d
Print "DOUBLE after bump d:"; d
showint 2.5
showint 3.5
showint d
showint 40000.4
n = 1
twice n
Print "after twice n:"; n
s$ = "abc"
Call addbang(s$)
Print "after CALL addbang(s$): "; s$
addbang (s$)
Print "after addbang (s$): "; s$
addbang s$ + "?"
Print "after addbang s$ + ?: "; s$
addbang "lit"
greet s$, 2.5
greet "x" + s$, n
System

Sub bump (x As Long)
    x = x + 1
    Print " in bump:"; x
End Sub

Sub showint (v As Integer)
    Print "showint:"; v
End Sub

Sub twice (y As Long)
    bump y
    bump y
End Sub

Sub addbang (t As String)
    t = t + "!"
    Print " in addbang: "; t
End Sub

Sub greet (t As String, k As Integer)
    Print "greet: "; t; k
End Sub
