$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x73_static_bound_len)
s: s
SYSTEM
SUB s
    STATIC v(LEN("abc")) AS LONG
    v(1) = v(1) + 1
    PRINT v(1); UBOUND(v)
END SUB
