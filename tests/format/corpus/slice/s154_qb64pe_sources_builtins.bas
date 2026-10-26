$Console:Only
' Slice (m4-qb64pe-sources task 7.2): what the upstream programs qb64pe/* needed. _INSTRREV with and without a
' start; _WRITEFILE and _READFILE$ round trip a file of the test's own folder (removed after); _FULLPATH$ (only
' what does not depend on the folder is printed); _ARCSEC, _ARCCSC, _ARCCOT, _SECH, _CSCH and one error 5; the ASC
' statement with and without a position and on an empty string; key$, exp$ and base$ as names; the $ name of a
' STRING * n array and variable in a SUB; a dotted name in a FUNCTION that does not share the TYPE variable; and
' STOP, which ends the program (its expression is not evaluated).
Type pn
    typ As Long
End Type
Dim num As pn
Dim Shared pathsep As String * 1
ReDim Shared names(1 To 3) As String * 8
On Error GoTo h
a$ = "a.b.c"
Print _InStrRev(a$, "."); _InStrRev(3, a$, "."); _InStrRev("abc", "x")
f$ = "s154.tmp"
_WriteFile f$, "x" + Chr$(0) + "yz"
d$ = _ReadFile$(f$)
Print Len(d$); Asc(d$, 2); Right$(d$, 2)
p$ = _FullPath$(f$)
Print Right$(_FullPath$("."), 1); Len(p$) > Len(f$); Right$(p$, Len(f$)) = f$
Kill f$
d$ = _ReadFile$(f$)
x# = 1.2
Print _Arcsec(x#); _Arccsc(x#); _Arccot(x#); _Sech(x#); _Csch(x#)
r# = _Arcsec(0.5)
b$ = "a/b/c"
Asc(b$, 2) = 92
Asc(b$) = 65 + 256
Print b$
e$ = ""
Asc(e$) = 66
Dim section$, key$
key$ = "k": section$ = "s"
Print join$(key$, section$)
setnames
Print names(2); "|"; pathsep; "|"; Len(names$(2))
num.typ = 3
Print notset&; num.typ
Print "stop"
Stop noisy
Print "not printed"
System
h:
Print "error"; Err
Resume Next
Function join$ (exp$, base$)
    join$ = exp$ + "=" + base$ + Str$(Exp(0))
End Function
Sub setnames
    pathsep$ = "\"
    names$(2) = "two"
    If Right$("a\", 1) = pathsep$ Then Print "sep"; UBound(names$)
End Sub
Function notset&
    notset& = num.typ + 1
End Function
Function noisy
    Print "noisy called"
End Function
