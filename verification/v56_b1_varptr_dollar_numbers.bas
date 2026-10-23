$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR$ of a scalar of each numeric type and of a string, printed
' as the code of each byte, beside VARPTR of the same variable; SADD of a string as it grows.
DIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64
DIM s AS SINGLE, d AS DOUBLE, f AS _FLOAT, o AS _OFFSET
DIM ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM t AS STRING, sa(1 TO 3) AS STRING
show "b", VARPTR$(b), VARPTR(b)
show "i", VARPTR$(i), VARPTR(i)
show "l", VARPTR$(l), VARPTR(l)
show "q", VARPTR$(q), VARPTR(q)
show "s", VARPTR$(s), VARPTR(s)
show "d", VARPTR$(d), VARPTR(d)
show "f", VARPTR$(f), VARPTR(f)
show "o", VARPTR$(o), VARPTR(o)
show "ub", VARPTR$(ub), VARPTR(ub)
show "ui", VARPTR$(ui), VARPTR(ui)
show "ul", VARPTR$(ul), VARPTR(ul)
show "uq", VARPTR$(uq), VARPTR(uq)
show "t", VARPTR$(t), VARPTR(t)
show "sa(2)", VARPTR$(sa(2)), VARPTR(sa(2))
show "x", VARPTR$(x), VARPTR(x)
PRINT LEN(VARPTR$(i))
PRINT "sadd"; SADD(t); LEN(t)
t = "abc"
PRINT "sadd"; SADD(t); LEN(t)
t = t + STRING$(100, "-")
PRINT "sadd"; SADD(t); LEN(t)
sa(2) = "element"
PRINT "sadd sa"; SADD(sa(2)); SADD(sa(1))
DEF SEG = VARSEG(t): PRINT PEEK(SADD(t)); PEEK(SADD(sa(2))): DEF SEG
SYSTEM

SUB show (n AS STRING, v AS STRING, p AS LONG)
    PRINT n; ":";
    FOR k = 1 TO LEN(v): PRINT ASC(v, k);: NEXT
    PRINT "|"; p MOD 256; p \ 256
END SUB
