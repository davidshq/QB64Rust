$Console:Only
' Arrays used without DIM (spec language/arrays-and-types, "Implicit arrays"): in the main module a static array,
' OPTION BASE to 10 per index written, of the suffix's type or the default type; LBOUND and UBOUND of a name that
' is no array make one; the names that stay free; ERASE and REDIM of one; below $DYNAMIC a dynamic array that the
' statement naming it first allocates; in a SUB or FUNCTION an array that nothing allocates (every use raises
' error 9). Error cases go to a handler that prints ERR and resumes next.
On Error GoTo h
Dim i As Long
z(10) = 4
Print "1. z(10) = 4: z(10), LBOUND, UBOUND"; z(10); LBound(z); UBound(z)
Print "2. z(11) = 1, z(-1):";
z(11) = 1
Print z(-1)
m(2, 3) = 5
Print "3. two indexes: UBOUND 1, UBOUND 2, LBOUND 2, m(2, 3), m(10, 10)"; UBound(m, 1); UBound(m, 2); LBound(m, 2); m(2, 3); m(10, 10)
t(1, 2, 3) = 1
Print "4. three indexes: UBOUND 3, t(1, 2, 3), t(10, 10, 10)"; UBound(t, 3); t(1, 2, 3); t(10, 10, 10)
s$(2) = "ab": i%(3) = 2.5: d#(1) = 1# / 3: q&&(10) = 5000000000: u~%%(4) = 200: b`(5) = 1
Print "5. suffixes: "; s$(2); UBound(s$); i%(3); d#(1); q&&(10); u~%%(4); b`(5); Len(i%(3)); Len(d#(1))
Print "6. the first use is a read:"; r(4); UBound(r); "["; rs$(1); "]"
Print "7. LBOUND(lb) first, then lb(10) = 3:"; LBound(lb); UBound(lb);
lb(10) = 3: Print lb(10);
Print " UBOUND(lb, 2):";
Print UBound(lb, 2)
Print "8. the first use skipped by GOTO:";
GoTo sk
w(1) = 1
sk:
Print UBound(w); w(1)
Print "9. REDIM z(20):";
ReDim z(20)
Print UBound(z); z(10)
Dim c(3) As Long
c!(7) = 1.5: c(1) = 2
Print "10. c!(7) beside DIM c(3) AS LONG: c!(7), c(1), UBOUND(c), UBOUND(c!)"; c!(7); c(1); UBound(c); UBound(c!)
a = 5: a(2) = 3
Print "11. a scalar and an array of one name: a, a(2)"; a; a(2)
Print "12. an index in a float and in a string of digits:"; z(9.6);
z(Val("3")) = 8: Print z(3)
Dim x As Long
x = 7: x(3) = 2.5
Print "13. beside DIM x AS LONG: x, x(3), LEN(x(3))"; x; x(3); Len(x(3))
Const k = 1
k(1) = 2
Print "14. beside CONST k: k, k(1)"; k; k(1)
chr(1) = 5: abc(2) = left(3) + 1: __foo(1) = 2
Print "15. chr(1), left(3), __foo(1), UBOUND(left):"; chr(1); left(3); __foo(1); UBound(left)
e(1) = 5: es$(2) = "ab"
Erase e, es$
Print "16. ERASE: e(1), UBOUND(e), es$(2):"; e(1); UBound(e); "["; es$(2); "]"
For i = 0 To 10: f(i) = i * i: Next
Swap f(2), f(3)
bump f(4)
Print "17. filled in a loop, SWAP, an element by reference:"; f(2); f(3); f(4); f(10)
ss$(0) = "x": ss$(10) = ss$(0) + "y"
Print "18. strings: "; ss$(10); Len(ss$(5))
DefInt A-Z
Option Base 1
di(1) = 2.5
Print "19. DEFINT A-Z and OPTION BASE 1: di(1), LEN, LBOUND, UBOUND, UBOUND(di%)"; di(1); Len(di(1)); LBound(di); UBound(di); UBound(di%)
Print "20. di(0):";
Print di(0)
dm(1, 1) = 1
Print "21. two indexes under base 1:"; LBound(dm, 1); LBound(dm, 2); UBound(dm, 2)
DefSng A-Z
Option Base 0
'$Dynamic
dy(3) = 4
Print "22. below $DYNAMIC: dy(3), LBOUND, UBOUND"; dy(3); LBound(dy); UBound(dy)
Print "23. the first use skipped by GOTO:";
GoTo sk2
dw(1) = 1
sk2:
Print UBound(dw);
dw(1) = 2
Print
Print "24. the first use run three times:";
For i = 1 To 3
    dl(3) = dl(3) + 1
Next
Print dl(3); UBound(dl)
Print "25. REDIM of it:";
ReDim dy(20)
Print UBound(dy); dy(3)
Print "26. a one-line IF taken, and one not taken:";
If i > 0 Then it(1) = 2
If i < 0 Then nt(1) = 2
Print it(1); UBound(it);
Print UBound(nt)
Print "27. LBOUND first:";
Print LBound(dlb); UBound(dlb)
Print "28. two in one statement:";
Print da(1) + db(1, 2); UBound(da); UBound(db, 2)
Print "29. ERASE of it, then a use:";
Erase da
Print da(1)
ds$(1) = "dyn"
Print "30. strings: "; ds$(1); UBound(ds$)
'$Static
st(1) = 6
Print "31. below $STATIC again:"; st(1); UBound(st)
ReDim st(3)
Print
Print "32. a SUB, called twice:";
plain
plain
Print
Print "33. a FUNCTION:";
i = fn(1)
Print
Print "34. beside main arrays, a parameter and a DIM SHARED scalar of the name:";
Dim Shared g As Long
g = 3
names 4
Print z(10); g
Print "35. ERASE in a SUB:";
wipe
Print

System
h:
Print " [error"; Err; "]";
Resume Next

Sub bump (v)
v = v + 100
End Sub
Sub plain
x(2) = x(2) + 1
Print " x(2):";
Print x(2);
End Sub
Function fn (n)
x(2) = 5
Print " UBOUND(x), LBOUND(x):";
Print UBound(x); LBound(x);
s$(1) = "a"
Print " ["; s$(1); "]";
End Function
Sub names (n)
Print " z(10):";
Print z(10);
g(1) = 2.5
Print " g(1):";
Print g(1);
n(1) = 1
Print " n(1), n:";
Print n(1); n
End Sub
Sub wipe
w2(2) = 5
Erase w2
Print " UBOUND(w2):";
Print UBound(w2);
End Sub
