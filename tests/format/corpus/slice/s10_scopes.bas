$Console:Only
' Procedure scopes (spec language/procedures, "Procedure scopes", "DECLARE lines"): locals (DIM and implicit) are
' new on every call; STATIC keeps its value; SHARED binds a main-module variable; DIM SHARED is seen only by
' procedures after it in the file; DECLARE lines change nothing.
DECLARE SUB counter ()
DECLARE FUNCTION getg& ()
Sub early
    Print "g in early:"; g
End Sub
Dim Shared g As Long
g = 10
early
x = 99
localx
localx
Print "main x:"; x
counter
counter
counter
Print "getg:"; getg&
Print "main g after getg:"; g
h& = 9
useshared
Print "main h& after useshared:"; h&
samename 5
Print "main y:"; y
System

Sub localx
    Print "x in localx:"; x
    x = 5
    Dim k As Long
    Print "k in localx:"; k
    k = 7
End Sub

Sub counter
    Static c As Long
    Static acc$
    c = c + 1
    acc$ = acc$ + "a"
    Print "count:"; c; acc$
End Sub

Function getg&
    Print "g in getg:"; g
    g = 20
    getg& = g * 2
End Function

Sub useshared
    Shared h As Long
    Print "h in useshared:"; h
    h = 5
End Sub

Sub samename (y As Long)
    Print "y in samename:"; y
End Sub
