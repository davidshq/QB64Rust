$Console:Only
' Default types and procedures (spec language/default-types): the table follows the file. A SUB and a FUNCTION
' above a statement keep the earlier default for parameter, variables and result; those below get the new one.
' A FUNCTION is called by its plain name whatever the default where the call stands, and with the suffix of its
' type. Parameters and results typed by a default, STRING and STRING * n included; STATIC and SHARED typed by it.
' The value 100000 + 1/3 shows a result's type: 100000 LONG, 100000.3359375 SINGLE, 100000.3333333333 DOUBLE.
Sub sa (p)
    v = 2.5
    Print "1. a SUB above every statement: LEN of its parameter and variable"; Len(p); Len(v)
End Sub
Function fa (x)
    fa = 100000 + 1# / 3
End Function
DefDbl A-Z
Dim r As Double
m = 2.5
Print "0. main below DEFDBL A-Z: LEN(m)"; Len(m)
sa 1
r = fa(3): Print "2. a FUNCTION above it, called plain and as fa!:"; r;
r = fa!(3): Print r
sb 1
r = fb(3): Print "4. a FUNCTION below DEFDBL A-Z, called plain and as fb#:"; r;
r = fb#(3): Print r
DefLng F
Print "5. FUNCTION f2 below DEFLNG F, as f2(3) and f2&(3):"; f2(3); f2&(3)
DefSng F
Print "6. f2(3) where the default for F is SINGLE again:"; f2(3)
DefInt A-Z
gv = 7
sub1
half 5
fx "abcdefgh"
Print "10. a FUNCTION typed STRING * 5 by _DEFINE: ["; fs; "]"
Print "11. a FUNCTION typed STRING by DEFSTR: "; sf(2); " "; sf$(3)
sv$ = "x"
show sv$
Print "    and what the SUB stored in its parameter: "; sv$
count
Print
count
Print
Print "14. gv after SHARED gv in the SUB:"; gv
System
DefDbl A-Z
Sub sb (p)
    v = 2.5
    Print "3. a SUB below DEFDBL A-Z: LEN of its parameter and variable"; Len(p); Len(v)
End Sub
Function fb (x)
    fb = 100000 + 1# / 3
End Function
DefLng F
Function f2 (x)
    f2 = x * 100000
End Function
DefInt A-Z
Sub sub1
    v = 2.5
    Print "7. a SUB below DEFINT A-Z: v = 2.5 is"; v
End Sub
Sub half (n)
    Print "8. its parameter n, called with 5: n / 2 is"; n / 2
End Sub
_Define F As STRING * 5 
Sub fx (f1)
    Print "9. a parameter typed STRING * 5 by _DEFINE: ["; f1; "]"; Len(f1)
End Sub
Function fs
    fs = "xy"
End Function
DefStr S
Function sf (n As Long)
    sf = String$(n, "r")
End Function
Sub show (s)
    Print "12. a parameter typed STRING by DEFSTR: "; s + "!"
    s = "changed"
End Sub
DefInt A-Z
Sub count
    Static st
    Shared gv
    st = st + 1.5
    gv = gv + 1
    Print "13. STATIC st and SHARED gv typed INTEGER:"; st; gv; Len(st);
End Sub
