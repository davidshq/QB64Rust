$CONSOLE:ONLY
' A DEFxxx inside a SUB (spec language/default-types, "Default types apply from the statement on"): above it the
' SUB's names have the default of the header; below it a plain name is the name plus the new default, so the
' parameter's and the variable's names mean new variables; the statement goes on applying after END SUB, to the
' main-module lines and the procedures below it in the file, also for a FUNCTION whose own name stays its result.
SUB inner (p)
    u = 2.5
    PRINT "1. above DEFINT A-Z in the SUB: LEN(p), LEN(u), p, u"; LEN(p); LEN(u); p; u
    DEFINT A-Z
    v = 2.5
    PRINT "2. below it: v, LEN(v), and p and u named again"; v; LEN(v); LEN(p); LEN(u); p; u
    PRINT "   the parameter and the first variable by their suffix:"; p!; u!
END SUB
w = 2.5
PRINT "3. a main-module line below the SUB: w, LEN(w)"; w; LEN(w)
inner 1
nxt 1
DIM r AS DOUBLE
r = f(3)
PRINT "5. a FUNCTION with DEFDBL A-Z inside, its plain name assigned below it:"; r
z = 1# / 3
PRINT "6. a main-module line above that FUNCTION in the file: z"; z
SYSTEM
SUB nxt (p)
    v = 2.5
    PRINT "4. the SUB below it: LEN(p), LEN(v)"; LEN(p); LEN(v)
END SUB
FUNCTION f (x)
    DEFDBL A-Z
    f = 100000 + 1# / 3
END FUNCTION
