$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x75_static_bound_function)
s: s
SYSTEM
FUNCTION f
    PRINT "f";
    f = 6
END FUNCTION
SUB s
    STATIC v(f) AS LONG
    v(1) = v(1) + 1
    PRINT v(1); UBOUND(v)
END SUB
