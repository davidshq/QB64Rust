$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7, review): what stands between an array parameter's parentheses is not read.
DIM x(3) AS LONG, m(2, 4) AS LONG
n = 2
x(1) = 7
one x()
two x()
three x()
four m()
five x()
SYSTEM
SUB one (a(n) AS LONG)
    PRINT a(1); UBOUND(a)
END SUB
SUB two (a(n + 1, "x") AS LONG)
    PRINT a(1); UBOUND(a)
END SUB
SUB three (a(-1) AS LONG)
    PRINT a(1); UBOUND(a)
END SUB
SUB four (a(1 TO 5) AS LONG)
    PRINT UBOUND(a, 1); UBOUND(a, 2)
END SUB
SUB five (a&(nothing declared here))
    PRINT a&(1); UBOUND(a&)
END SUB
