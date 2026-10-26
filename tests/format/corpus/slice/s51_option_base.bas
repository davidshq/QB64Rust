$Console:Only
' OPTION BASE with static arrays (spec language/arrays-and-types, "OPTION BASE"): the lower bound of a dimension
' written without TO, from the statement on in the file; bounds with TO unchanged; the number 0 in plain digits
' staying 0 to 0, per dimension; base 0 again; a statement skipped by GOTO and one inside a SUB still applying
' below them; what follows the word after BASE ignored; an index below the base (error 9); arrays of strings and
' of a user type. The other kinds of array come with their own slice programs.
Type rec
    n As Long
End Type
On Error GoTo h
Dim a0(3)
Option Base 1
Dim a(3): Dim b(2 To 4)
Print "1. base 1, DIM a(3), DIM b(2 TO 4): LBOUND(a), UBOUND(a), LBOUND(b)"; LBound(a); UBound(a); LBound(b)
Print "2. DIM a0(3) above the statement:"; LBound(a0); UBound(a0)
Dim z(0)
Print "3. DIM z(0):"; LBound(z); UBound(z)
Dim z2(0, 3), z3(3, 0), z4(0)
Print "4. DIM z2(0, 3):"; LBound(z2, 1); UBound(z2, 1); LBound(z2, 2); UBound(z2, 2)
Print "   DIM z3(3, 0):"; LBound(z3, 1); UBound(z3, 1); LBound(z3, 2); UBound(z3, 2)
Print "   DIM z4(00):"; LBound(z4); UBound(z4)
Dim s(2) As String, r(2) As rec, m(2, 1 To 2) As Integer
s(1) = "one": s(2) = "two": r(1).n = 5: m(1, 1) = 7: m(2, 2) = 8
Print "5. strings, a user type, two dimensions: "; s(1); " "; s(2); r(1).n; r(2).n; m(1, 1); m(2, 2); LBound(m, 1); LBound(m, 2)
Print "6. an index below the base, a(0) = 1:";
a(0) = 1
Print " and PRINT s(0):";
Print s(0)
a(1) = 1.5: a(3) = 3.5
Print "7. a(1), a(3):"; a(1); a(3)
Option Base 0
Dim c(3)
Print "8. base 0 again, DIM c(3):"; LBound(c); UBound(c)
GoTo skip
Option Base 1
skip:
Dim e(3)
Print "9. DIM e(3) below an OPTION BASE 1 that GOTO skipped:"; LBound(e)
Option Base 0
Dim f(3)
Print "10. DIM f(3) below OPTION BASE 0 + 1:"; LBound(f)
Option Base 1
Dim g(3)
Print "11. DIM g(3) below OPTION BASE 1 x:"; LBound(g)
Option Base 0
System
h:
Print " [error"; Err; "]";
Resume Next
