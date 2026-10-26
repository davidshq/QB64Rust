$Console:Only
' Array parameters (spec language/procedures, "Array parameters"): a parameter written name() takes a whole array
' by reference, static or dynamic, allocated or not, the procedure's own parameter passed on too. The procedure
' reads, stores, DIMs, REDIMs and ERASEs the caller's array, with the errors that array's kind gives. Signedness
' is not compared. The number of dimensions comes from the indexes in the procedure and from the arrays passed,
' above or below it. Error cases go to a handler that prints ERR and resumes next.
Type pt
    x As Long
    y As Integer
End Type
Sub above (m() As Long)
    Print "9. a SUB above the call, UBOUND(m, 2):"; UBound(m, 2)
End Sub
On Error GoTo h
Dim i As Long
Dim x(1 To 3) As Long, u(3) As _Unsigned Long, y(2, 4) As Long, e(3) As Long
Dim s(1 To 2) As String, f(2) As String * 3, p(2) As pt, b(3) As _Bit * 4, d2(2 To 5, -1 To 1) As Double
Dim Shared sh(3) As Long
ReDim d(2) As Long, r(3) As Long, never(0) As Long, gone(0) As Long
Common c() As Long
Erase never, gone
fill x()
Print "1. elements changed in the SUB:"; x(1); x(2); x(3)
d(2) = 5
grow d()
Print "2. REDIM _PRESERVE in the SUB:"; UBound(d); d(2)
outer x()
Print "3. passed on:"; x(1)
signed u()
Print "4. an _UNSIGNED LONG array as a LONG parameter:"; u(1)
Print "5. REDIM of a static argument:";
grow x()
Print UBound(x)
e(1) = 7: r(1) = 7
wipe e()
wipe r()
Print "6. ERASE in the SUB, static:"; UBound(e); e(1); " dynamic:";
Print UBound(r);
Print
Print "7. a SUB below the call, UBOUND(m, 2):";
below y()
Print
chain1 y()
above y()
Print "10. DIM of the parameter:";
dimmed x()
dimmed never()
Print UBound(x); UBound(never)
tell s(), f(), p(), b()
Print "11. strings, fixed strings, a TYPE, _BIT: "; s(1); s(2); "["; f(1); "]"; p(2).x; p(2).y; b(1)
Print "12. a FUNCTION:"; total#(d2()); d2(5, 1); pick(x(), 2)
Call fill(x())
Print "13. CALL fill(x()):"; x(3)
fill fresh&()
Print "14. an array never declared:"; fresh&(1); UBound(fresh&)
make c()
Print "15. a COMMON array given its bounds in the SUB:"; UBound(c, 1); UBound(c, 2); c(1, 2)
Print "16. a STATIC k() array passed on:";
keeper
keeper
Print
countdown x(), 3
Print "17. a recursive SUB:"; x(1); x(2); x(3)
ReDim d(1 To 4) As Long
pair d(), d()
Print "18. one array as two arguments:"; d(1); UBound(d)
Print "19. an index outside, in the SUB:";
outside x()
Print x(1)
sh(1) = 4
Print "20. a shared and a local array passed from a SUB:";
inner
Print sh(1)
Print "21. a parameter that is not allocated:";
Print bounds&(gone())
ReDim gone(6) As Long
Print "22. and allocated:"; bounds&(gone())
For i = 1 To 2
    unsign x()
Next
Print "23. a LONG array as an _UNSIGNED LONG parameter:"; x(1)
System
h:
Print " [error"; Err; "]";
Resume Next

Sub fill (a() As Long)
    Dim i As Long
    For i = LBound(a) To UBound(a)
        a(i) = i * 2
    Next
End Sub
Sub grow (a() As Long)
    ReDim _Preserve a(10) As Long
End Sub
Sub outer (a() As Long)
    middle a()
End Sub
Sub middle (b() As Long)
    b(1) = 4
End Sub
Sub signed (a() As Long)
    a(1) = -1
End Sub
Sub unsign (a() As _Unsigned Long)
    a(1) = a(1) + 4294967295
End Sub
Sub wipe (a() As Long)
    Erase a
End Sub
Sub below (m() As Long)
    Print UBound(m, 2);
End Sub
Sub chain1 (a() As Long)
    chain2 a()
End Sub
Sub chain2 (b() As Long)
    chain3 b()
End Sub
Sub chain3 (c() As Long)
    Print "8. down a chain of three:"; UBound(c, 1); UBound(c, 2)
End Sub
Sub dimmed (a() As Long)
    Dim a(5) As Long
End Sub
Sub tell (a() As String, b() As String * 3, c() As pt, e() As _Bit * 4)
    a(1) = "one": a(2) = a(1) + "+"
    b(1) = "abcdef"
    c(2).x = 70000: c(2).y = Len(b(1))
    e(1) = 5
End Sub
Function total# (m() As Double)
    Dim i As Long, j As Long, t As Double
    For i = LBound(m, 1) To UBound(m, 1)
        For j = LBound(m, 2) To UBound(m, 2)
            m(i, j) = i * j
            t = t + m(i, j)
        Next
    Next
    total# = t
End Function
Function pick (a() As Long, i As Long)
    pick = a(i)
End Function
Sub make (a() As Long)
    ReDim a(3, 4) As Long
    a(1, 2) = 7
End Sub
Sub keeper
    Static k() As Long
    grow3 k()
    k(1) = k(1) + 1
    Print UBound(k); k(1);
End Sub
Sub grow3 (a() As Long)
    ReDim _Preserve a(3) As Long
End Sub
Sub countdown (a() As Long, n As Long)
    If n = 0 Then Exit Sub
    a(n) = n * 10
    countdown a(), n - 1
End Sub
Sub pair (p() As Long, q() As Long)
    p(1) = 1: q(1) = q(1) + 1
    ReDim q(1 To 2) As Long
End Sub
Sub outside (a() As Long)
    a(99) = 1
    a(1) = 3
End Sub
Sub inner
    Dim mine(2) As Long
    fill sh()
    fill mine()
    Print mine(2);
End Sub
Function bounds& (a() As Long)
    bounds& = UBound(a)
End Function
