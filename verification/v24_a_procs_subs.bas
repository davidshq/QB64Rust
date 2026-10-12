$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): which SUBs a DEFxxx of the main module reaches (one above it in
' the file, one below), for a parameter and a local variable; a DEFxxx inside a SUB: before it, after it, and
' in the next SUB. LEN shows the type: 4 SINGLE, 8 DOUBLE, 2 INTEGER.
SUB sa (p)
    v = 2.5: PRINT "sa, above DEFDBL A-Z in the file: p, v"; LEN(p); LEN(v)
END SUB
DEFDBL A-Z
m = 2.5: PRINT "main after DEFDBL A-Z: m"; LEN(m)
sa 1
sb 1
sc 1
sd 1
n = 2.5: PRINT "main, a new variable below the calls: n"; LEN(n)
SYSTEM
SUB sb (p)
    v = 2.5: PRINT "sb, below DEFDBL A-Z: p, v"; LEN(p); LEN(v)
END SUB
SUB sc (p)
    u = 2.5
    PRINT "sc, before its DEFINT A-Z: p, u, and their values"; LEN(p); LEN(u); p; u
    DEFINT A-Z
    v = 2.5: PRINT "sc, after it: v, and p and u named again"; LEN(v); LEN(p); LEN(u); p; u
END SUB
SUB sd (p)
    v = 2.5: PRINT "sd, below sc: p, v"; LEN(p); LEN(v)
END SUB
