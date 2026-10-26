$Console:Only
' Arrays in procedures (spec language/arrays-and-types, "Arrays in procedures"): DIM and REDIM in a SUB or
' FUNCTION make a dynamic array that is new on every call; STATIC a(n) keeps its elements; STATIC a() lists a
' dynamic array that a DIM, a REDIM or a REDIM _PRESERVE below it allocates; SHARED a() names the main module's
' array; a shared main array is seen below its declaration in the file only; a procedure with STATIC after its
' header keeps its variables and arrays down to its first STATIC statement. Error cases go to a handler that
' prints ERR and resumes next.
Sub above
    Print "13. a SUB above the DIM SHARED: sa(1)";
    Print sa(1)
End Sub
On Error GoTo h
Dim v As Long
Dim g(3) As Long, hd(3), ns(3) As Long, bs(3) As String
ReDim d(3) As Long
Dim Shared sa(3) As Long
ReDim Shared sr(3) As Long
Print "1. DIM t(5), new on each call:"
fresh 5
fresh 5
Print "2. one DIM run twice in a call:";
twice
Print "3. STATIC c(3):";
keep
keep
keep
Print
Print "4. STATIC l() then DIM l(3):";
listdim
listdim
Print
Print "5. STATIC r() then REDIM r(3):";
listredim
listredim
Print
Print "6. STATIC k() then REDIM _PRESERVE k(2, 3):";
listpres
listpres
Print
Print "7. SUB ... STATIC, DIM a(2):";
substatic
substatic
Print
Print "8. SUB ... STATIC, DIM b(n):";
substaticdyn 2
substaticdyn 2
Print
Print "9. REDIM in a SUB, twice in a call:";
local 3
local 4
Print
g(2) = 8: hd(2) = 1.5: d(2) = 4: ns(1) = 5: bs(1) = "main"
show
Print "11. main after it: g(2), hd(2), UBOUND(d), d(5), ns(1), ns(2), bs(1): "; g(2); hd(2); UBound(d); d(5); ns(1); ns(2); bs(1)
sa(1) = 1: sr(1) = 2
Print "12. a SUB below DIM SHARED and REDIM SHARED:";
below
Print sa(1); sr(1); UBound(sr)
above
Print
Print "14. EXIT SUB with a local array, called twice:";
early 1
early 1
Print
Print "15. an error inside, then the SUB goes on:";
fails
fails
Print
Print "16. a recursive FUNCTION, each level with DIM a(n):";
v = down(3)
Print " result"; v
Print "17. a FUNCTION with a local string array: "; words("a"); words("b")
Print "18. used without DIM, then DIM of it:";
later
later
Print
Print "19. STATIC y(), used with no DIM:";
listed
listed
Print
Print "20. DIM of a shared main array in a SUB:";
again
Print sa(1); UBound(sa); UBound(sr)
Print "21. SUB ... STATIC: what is kept above and below its STATIC statement:"
scope
scope
scope
Print "22. FUNCTION ... STATIC:";
v = count(2)
v = count(2)
Print v
Print "23. a used array in a SUB ... STATIC:";
auto
auto
Print
Print "24. ERASE of a local, a STATIC and a listed array:";
wipe
wipe
Print
Print "25. DIM a() and REDIM b() in a SUB are scalars:";
empty
empty
Print
Print "26. two dimensions, a FOR over a local array:";
grid 3

System
h:
Print " [error"; Err; "]";
Resume Next

Sub fresh (n As Long)
    Dim t(5) As Long, v(n) As Long, s(2) As String, f(2) As String * 2
    Print "   t(1), v(n), s(1), ASC(f(1)), UBOUND(v):"; t(1); v(n); "["; s(1); "]"; Asc(f(1)); UBound(v)
    t(1) = 7: v(n) = 8: s(1) = "old": f(1) = "zz"
End Sub
Sub twice
    Dim i As Long
    For i = 1 To 2
        Dim d(3) As Long
        d(1) = d(1) + 1
    Next
    Print d(1)
End Sub
Sub keep
    Static c(3) As Long, w(1 To 2) As String
    c(1) = c(1) + 1: w(1) = w(1) + "x": Print c(1); w(1);
End Sub
Sub listdim
    Static l() As Long
    Dim l(3) As Long
    l(1) = l(1) + 1: Print l(1);
End Sub
Sub listredim
    Static r() As Long
    ReDim r(3) As Long
    r(1) = r(1) + 1: Print r(1);
End Sub
Sub listpres
    Static k() As Long
    ReDim _Preserve k(2, 3) As Long
    k(2, 3) = k(2, 3) + 1: Print k(2, 3);
End Sub
Sub substatic Static
    Dim a(2) As Long
    a(1) = a(1) + 1: Print a(1);
End Sub
Sub substaticdyn (n As Long) Static
    Dim b(n) As Long
    b(1) = b(1) + 1: Print b(1);
End Sub
Sub local (n As Long)
    ReDim q(n) As Long
    q(1) = q(1) + 1
    ReDim q(n + 1) As Long
    Print UBound(q); q(1);
End Sub
Sub show
    Shared g() As Long
    Shared hd(), bs() As String
    Shared d() As Long
    Print "10. SHARED g(), hd(), d(), bs(): "; g(2); hd(2); d(2); bs(1)
    g(2) = 9: hd(2) = 2.5: bs(1) = "sub"
    ReDim d(5) As Long
    d(5) = 6
    Print "    ns(1) of a main array that is not shared:";
    Print ns(1)
    Dim ns(8) As Long
    ns(2) = 3
    Print "    after a local DIM ns(8): UBOUND, ns(1), ns(2)"; UBound(ns); ns(1); ns(2)
End Sub
Sub below
    Print sa(1); sr(1);
    sa(1) = 3
    ReDim sr(6) As Long
    sr(1) = 4
End Sub
Sub early (n As Long)
    Dim a(3) As Long, s(3) As String
    a(1) = a(1) + 1: s(1) = s(1) + "e"
    Print a(1); s(1);
    If n = 1 Then Exit Sub
    Print "not reached"
End Sub
Sub fails
    Dim a(3) As Long
    a(1) = a(1) + 1
    a(9) = 1
    Error 5
    Print a(1);
End Sub
Function down (n As Long)
    Dim a(n) As Long
    a(n) = n
    If n > 1 Then down = down(n - 1) + a(n) Else down = a(n)
    Print UBound(a); a(n);
End Function
Function words$ (s As String)
    Dim w(2) As String
    w(1) = w(1) + s + s
    words$ = "[" + w(1) + w(2) + "]"
End Function
Sub later
    a(1) = 5
    Dim a(3)
    a(1) = a(1) + 1
    Print a(1); UBound(a);
End Sub
Sub listed
    Static y() As Double
    y(2) = y(2) + 1
    Print Len(y(2));
End Sub
Sub again
    Dim sa(5) As Long
    Dim sr(9) As Long
End Sub
Sub scope Static
    Dim d As Long
    Dim a(2) As Long
    n = n + 1: d = d + 2: s$ = s$ + "a": a(1) = a(1) + 3
    Print "    above:"; n; d; s$; a(1);
    Static x(2) As Long
    Dim e As Long
    Dim w(2) As Long
    m = m + 1: e = e + 2: t$ = t$ + "b": x(1) = x(1) + 4: w(1) = w(1) + 5: n = n + 10
    Print " below:"; m; e; t$; x(1); w(1); n
End Sub
Function count (a As Long) Static
    c = c + a
    count = c
    Print c;
End Function
Sub auto Static
    u(2) = u(2) + 1
    Print u(2); UBound(u);
End Sub
Sub wipe
    Dim a(2) As Long
    Static c(2) As Long, l() As Long
    ReDim _Preserve l(2) As Long
    a(1) = a(1) + 1: c(1) = c(1) + 1: l(1) = l(1) + 1
    Print a(1); c(1); l(1);
    Erase a, c, l
    Print c(1);
    Print a(1);
    Print l(1);
End Sub
Sub empty
    Dim a As Long
    ReDim b As Long
    a = a + 1: b = b + 2
    Print a; b;
End Sub
Sub grid (n As Long)
    Dim m(1 To n, 1 To n) As Long, i As Long, j As Long, total As Long
    For i = 1 To n
        For j = 1 To n
            m(i, j) = i * j
        Next
    Next
    For i = 1 To n
        total = total + m(i, n)
    Next
    Print total; LBound(m, 2); UBound(m, 2)
End Sub
