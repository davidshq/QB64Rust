$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-names-implicit-arrays row): STATIC a() in a SUB beside a shared array a.
ON ERROR GOTO h
DIM SHARED a(3)
a(1) = 5
t
PRINT "main:"; a(1); UBOUND(a)
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT

SUB t
    STATIC a()
    PRINT "in t:"; UBOUND(a)
END SUB
