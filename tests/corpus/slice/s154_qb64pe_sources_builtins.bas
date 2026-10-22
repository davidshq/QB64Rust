$CONSOLE:ONLY
' Slice (m4-qb64pe-sources task 7.2): what the upstream programs qb64pe/* needed. _INSTRREV with and without a
' start; _WRITEFILE and _READFILE$ round trip a file of the test's own folder (removed after); _FULLPATH$ (only
' what does not depend on the folder is printed); _ARCSEC, _ARCCSC, _ARCCOT, _SECH, _CSCH and one error 5; the ASC
' statement with and without a position and on an empty string; key$, exp$ and base$ as names; the $ name of a
' STRING * n array and variable in a SUB; a dotted name in a FUNCTION that does not share the TYPE variable; and
' STOP, which ends the program (its expression is not evaluated).
TYPE pn
    typ AS LONG
END TYPE
DIM num AS pn
DIM SHARED pathsep AS STRING * 1
REDIM SHARED names(1 TO 3) AS STRING * 8
ON ERROR GOTO h
a$ = "a.b.c"
PRINT _INSTRREV(a$, "."); _INSTRREV(3, a$, "."); _INSTRREV("abc", "x")
f$ = "s154.tmp"
_WRITEFILE f$, "x" + CHR$(0) + "yz"
d$ = _READFILE$(f$)
PRINT LEN(d$); ASC(d$, 2); RIGHT$(d$, 2)
p$ = _FULLPATH$(f$)
PRINT RIGHT$(_FULLPATH$("."), 1); LEN(p$) > LEN(f$); RIGHT$(p$, LEN(f$)) = f$
KILL f$
d$ = _READFILE$(f$)
x# = 1.2
PRINT _ARCSEC(x#); _ARCCSC(x#); _ARCCOT(x#); _SECH(x#); _CSCH(x#)
r# = _ARCSEC(0.5)
b$ = "a/b/c"
ASC(b$, 2) = 92
ASC(b$) = 65 + 256
PRINT b$
e$ = ""
ASC(e$) = 66
DIM section$, key$
key$ = "k": section$ = "s"
PRINT join$(key$, section$)
setnames
PRINT names(2); "|"; pathsep; "|"; LEN(names$(2))
num.typ = 3
PRINT notset&; num.typ
PRINT "stop"
STOP noisy
PRINT "not printed"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
FUNCTION join$ (exp$, base$)
    join$ = exp$ + "=" + base$ + STR$(EXP(0))
END FUNCTION
SUB setnames
    pathsep$ = "\"
    names$(2) = "two"
    IF RIGHT$("a\", 1) = pathsep$ THEN PRINT "sep"; UBOUND(names$)
END SUB
FUNCTION notset&
    notset& = num.typ + 1
END FUNCTION
FUNCTION noisy
    PRINT "noisy called"
END FUNCTION
