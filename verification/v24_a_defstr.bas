$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DEFSTR for an array, a parameter, a FUNCTION result, DIM,
' STATIC, and a number stored through a suffix.
DEFSTR S
DIM st(3)
st(1) = "one": PRINT "array st(1): "; st(1); LEN(st(1))
DIM sd
sd = "dim": PRINT "DIM sd: "; sd
sv = "x"
show sv
PRINT "after the SUB changed its parameter: "; sv
PRINT "FUNCTION sf: "; sf(2); " "; sf$(3)
s% = 5: PRINT "s% beside the STRING default:"; s%
keep: keep
SYSTEM
SUB show (s)
    PRINT "parameter s: "; s + "!"
    s = "changed"
END SUB
FUNCTION sf (n AS LONG)
    sf = STRING$(n, "r")
END FUNCTION
SUB keep
    STATIC sk
    sk = sk + "k": PRINT "STATIC sk: "; sk
END SUB
