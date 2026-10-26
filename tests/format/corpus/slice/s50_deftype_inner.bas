$Console:Only
' A DEFxxx inside a SUB (spec language/default-types, "Default types apply from the statement on"): above it the
' SUB's names have the default of the header; below it a plain name is the name plus the new default, so the
' parameter's and the variable's names mean new variables; the statement goes on applying after END SUB, to the
' main-module lines and the procedures below it in the file, also for a FUNCTION whose own name stays its result.
Sub inner (p)
    u = 2.5
    Print "1. above DEFINT A-Z in the SUB: LEN(p), LEN(u), p, u"; Len(p); Len(u); p; u
    DefInt A-Z
    v = 2.5
    Print "2. below it: v, LEN(v), and p and u named again"; v; Len(v); Len(p); Len(u); p; u
    Print "   the parameter and the first variable by their suffix:"; p!; u!
End Sub
w = 2.5
Print "3. a main-module line below the SUB: w, LEN(w)"; w; Len(w)
inner 1
nxt 1
Dim r As Double
r = f(3)
Print "5. a FUNCTION with DEFDBL A-Z inside, its plain name assigned below it:"; r
z = 1# / 3
Print "6. a main-module line above that FUNCTION in the file: z"; z
System
Sub nxt (p)
    v = 2.5
    Print "4. the SUB below it: LEN(p), LEN(v)"; Len(p); Len(v)
End Sub
Function f (x)
    DefDbl A-Z
    f = 100000 + 1# / 3
End Function
