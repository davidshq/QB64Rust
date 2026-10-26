$Console:Only
' ERASE (spec language/arrays-and-types, "ERASE"): a static array is cleared and keeps its bounds, of every element
' type and in two dimensions; a dynamic array is released (its bounds and elements raise error 9), a second ERASE
' does nothing, and REDIM or DIM allocate it again; several names in one statement, in order; a suffix that names
' the array; a scalar of the array's name unchanged; ERASE of a shared array in a SUB. Error cases go to a handler
' that prints ERR and resumes next.
Type rec
    n As Long
    t As String * 2
    d As Double
End Type
On Error GoTo h
Dim i As Long, n As Long
n = 4
Dim a(5) As Long
a(2) = 9
Erase a
Print "1. static LONG: UBOUND, a(2)"; UBound(a); a(2)
Dim st(2) As String, fx(2) As String * 3, ud(2) As rec, db(1 To 2, 3) As Double, b4(3) As _Bit * 4
st(1) = "one": fx(1) = "abc": ud(1).n = 7: ud(1).t = "xy": ud(1).d = 1.5: db(2, 3) = 2.5: b4(1) = 7
Erase st, fx, ud, db, b4
Print "2. static STRING, STRING * 3, a user type, DOUBLE in two dimensions, _BIT * 4: ["; st(1); "]"; Len(st(1)); Len(fx(1)); Asc(fx(1)); ud(1).n; Asc(ud(1).t); ud(1).d; db(2, 3); b4(1); LBound(db, 1); UBound(db, 2)
st(1) = "again": b4(1) = 3
Print "   and stored again: "; st(1); b4(1)
ReDim d(5) As Long
d(1) = 4
Erase d
Print "3. dynamic: UBOUND(d)";
Print UBound(d)
Print "   LBOUND(d)";
Print LBound(d)
Print "   d(1)";
Print d(1)
Print "   d(1) = 1";
d(1) = 1
Print
Print "   ERASE d again:";
Erase d
Print " nothing"
ReDim d(2) As Long
Print "   REDIM d(2): UBOUND, d(1)"; UBound(d); d(1)
Print "4. DIM z(n), ERASE z and the same DIM again, in a loop:";
For i = 1 To 2
    Dim z(n + i) As Long
    z(1) = z(1) + i
    Print UBound(z); z(1);
    Erase z
Next
Print
ReDim ds(2) As String, df(2) As String * 3, du(2) As rec, dbt(2) As _Bit * 4
ds(1) = "abc": df(1) = "abc": du(1).n = 7: dbt(1) = 3
Erase ds, df, du, dbt
Print "5. dynamic STRING, STRING * 3, a user type, _BIT * 4: UBOUND of each";
Print UBound(ds)
Print UBound(df)
Print UBound(du)
Print UBound(dbt)
ReDim ds(1) As String, df(1) As String * 3, du(1) As rec, dbt(1) As _Bit * 4
Print
Print "   after REDIM of each: ["; ds(1); "]"; Asc(df(1)); du(1).n; dbt(1)
Dim sc&(3)
sc& = 5: sc&(1) = 6
Erase sc&
Print "6. a scalar of the array's name: sc&, sc&(1)"; sc&; sc&(1)
Dim sf(3) As Long
sf(1) = 5
Erase sf&
Print "7. the array by its suffix, ERASE sf&:"; sf(1)
ReDim m1(2) As Long
Dim m2(2) As Long
m1(1) = 1: m2(1) = 2
Erase m1, m2, m1
Print "8. a dynamic and a static array in one statement, the dynamic one twice:"; m2(1); UBound(m2);
Print UBound(m1)
Print
Dim Shared gs(3) As Long
ReDim Shared gd(3) As Long
gs(1) = 1: gd(1) = 2
wipe
Print "9. ERASE of shared arrays in a SUB:"; gs(1); UBound(gs);
Print UBound(gd)
System
h:
Print " [error"; Err; "]";
Resume Next

Sub wipe
    Erase gs, gd
End Sub
