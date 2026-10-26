$Console:Only
' Slice program (m2-arrays-and-types, D6, D9): the store rule of each place, the order parts of a statement are
' evaluated in, and which error ERR reports. Errors are raised by a bad index (9) and by CHR$(-1) inside INSTR
' (5, placeholder 0). Where the new compiler differs on purpose (DIVERGENCES.md D-004: a member store into an
' element with a bad index writes element 0 in the old compiler) this program does not look: element 0 of `bad`
' is never printed. No PRINT comma.
Type t
    m As Long
    n As Long
End Type
Dim x(10) As Long
Dim s(3) As String
Dim u As t
Dim a(3) As t
Dim bad(3) As t
Dim mm(3, 3) As Long
Dim k As Long
k = -1
On Error GoTo h

' Element stores: the index first; with an error pending after it nothing is stored.
x(9) = 5: x(9) = InStr(Chr$(k), "x")
Print "x(9) = raising value:"; x(9)
x(0) = 77: x(10) = 88
x(11) = 5
Print "after x(11) = 5:"; x(0); x(10)
x(-1) = 6
Print "after x(-1) = 6:"; x(0); x(10)
x(11) = InStr(Chr$(k), "x")
Print "after x(11) = raising value:"; x(0); x(10)
x(InStr(Chr$(k), "x")) = 5
Print "after x(raising index) = 5:"; x(0)
s(0) = "zero"
s(4) = "four"
Print "after s(4) = four: ["; s(0); "] ["; s(3); "]"
s(1) = "a": s(1) = Chr$(k)
Print "s(1) = CHR$(-1): ["; s(1); "]"

' A read with a bad index gives element 0's value.
y = 5: y = x(11)
Print "y = x(11):"; y
x(2) = 4: x(2) = x(-1) + 1
Print "x(2) = x(-1) + 1:"; x(2)
x(3) = 4: x(x(11) + 3) = 9
Print "x(x(11) + 3) = 9:"; x(0); x(3)
t$ = "t": t$ = s(9)
Print "t$ = s(9): ["; t$; "]"
Print "print"; x(11); "rest"
Print "after print"

' Member stores are not guarded.
u.m = 5: u.m = InStr(Chr$(k), "x")
Print "u.m = raising value:"; u.m
a(2).n = 3: a(2).n = InStr(Chr$(k), "x")
Print "a(2).n = raising value:"; a(2).n
a(0).n = 60
a(1).n = 4: a(1).n = a(9).n
Print "a(1).n = a(9).n:"; a(1).n

' A member store into an element: the value before the index; the first error wins.
bad(1).m = 71: bad(2).m = 72: bad(3).m = 73
bad(9).m = 5
Print "after bad(9).m = 5:"; bad(1).m; bad(2).m; bad(3).m
bad(9).m = InStr(Chr$(k), "x")
Print "after bad(9).m = raising value:"; bad(1).m; bad(2).m; bad(3).m
a(ix&(1)).m = va&(5)
Print "a(1).m:"; a(1).m
x(ix&(2)) = va&(6)
Print "x(2):"; x(2)
y& = x(ix&(2)) + va&(0)
Print "y&:"; y&

' Several indexes: left to right, each checked on its own, first error wins.
mm(ix&(1), ix&(2)) = 5
Print "mm(1, 2):"; mm(1, 2)
Print "mm(1, 4):"; mm(1, 4)
mm(9, InStr(Chr$(k), "x")) = 1
Print "after mm(9, raising) = 1"
mm(InStr(Chr$(k), "x"), 9) = 1
Print "after mm(raising, 9) = 1"

' By reference: a bad index never reaches the SUB; arguments left to right.
x(0) = 50: bump x(11)
Print "bump x(11):"; x(0)
bump bad(9).m
Print "after bump bad(9).m:"; bad(1).m
two x(11), InStr(Chr$(k), "x")
Print "after two x(11), raising"
two InStr(Chr$(k), "x"), x(11)
Print "after two raising, x(11)"

' LBOUND and UBOUND with a dimension out of range.
Print "LBOUND(mm, 3):"; LBound(mm, 3)
Print "UBOUND(x, 0):"; UBound(x, 0)
System

h:
Print "handler"; Err
Resume Next

Sub bump (v As Long)
    Print "in bump"; v
    v = v + 1
End Sub

Sub two (p As Long, q As Long)
    Print "in two"; p; q
End Sub

Function ix& (n As Long)
    Print "index"; n
    ix& = n
End Function

Function va& (n As Long)
    Print "value"; n
    va& = n
End Function
