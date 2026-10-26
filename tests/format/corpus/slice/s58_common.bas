$Console:Only
' COMMON (spec language/arrays-and-types, "COMMON"): COMMON declares each variable as DIM does and COMMON SHARED as
' DIM SHARED does, wherever it stands in the main module. COMMON name() lists a dynamic array that is not
' allocated; what stands between its parentheses is not read. A DIM of that name and type below allocates it when
' it runs, a REDIM gives it new bounds, and a use before either raises error 9. Error cases go to a handler that
' prints ERR and resumes next.
Type pt
    x As Long
End Type
On Error GoTo h
Dim n As Long, i As Long
n = 3
Common Shared total As Long, word$, w As pt, fx As String * 4
Common v As Long
Common As Long la, lb()
Common Shared cs() As Long, ign() As Long, other()
Common sc, never() As Double, twice() As Long
total = 5: word$ = "w": w.x = 6: fx = "abcdef": v = 7
Print "1. COMMON SHARED scalars, seen in a SUB:";
scalars
Print "2. COMMON alone is not shared:";
unshared
Print "3. COMMON cs(), used before its DIM:";
cs(2) = 6
Print
Dim cs(n) As Long
cs(2) = 6
Print "4. COMMON SHARED cs() then DIM cs(n), in a SUB:";
arrays
Print "5. REDIM of it:";
ReDim cs(9) As Long
Print UBound(cs); cs(2)
Print "6. COMMON ign(3) allocates nothing:";
Print UBound(ign);
Print
Dim Shared ign(2, 5) As Long
ign(1, 2) = 8
Print "7. DIM SHARED ign(2, 5) below it:"; UBound(ign, 1); UBound(ign, 2);
arrays2
Dim other(3) As Long
other(2) = 6
Print "8. COMMON SHARED other() then DIM other(3) AS LONG, in a SUB:";
another
Print
Print "9. COMMON never(), never given bounds:";
never(1) = 1
Print UBound(never);
Print
Print "10. the type before the names:";
Dim lb(1 To 2) As Long
la = 100000: lb(2) = 200000
Print la; lb(2); LBound(lb)
Dim sc(3)
sc = 1.5: sc(1) = 2.5
Print "11. COMMON sc then DIM sc(3):"; sc; sc(1)
Print "12. DIM of a COMMON array, run twice:";
For i = 1 To 2
    Dim twice(i + 1) As Long
Next
Print UBound(twice)
Common Shared below
below = 9
Print "13. COMMON below statements:";
late
Erase cs
Print "14. ERASE of a COMMON array:";
Print UBound(cs);
Print
System
h:
Print " [error"; Err; "]";
Resume Next

Sub scalars
    Print total; word$; w.x; fx
    total = total + 1
End Sub
Sub unshared
    Print v
End Sub
Sub arrays
    Print cs(2); UBound(cs)
End Sub
Sub arrays2
    Print ign(1, 2)
End Sub
Sub another
    Print other(2);
End Sub
Sub late
    Print below
End Sub
