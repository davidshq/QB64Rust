$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): the order variables named by VARPTR take conventional memory:
' DIM, implicit variables, a CONST between, DIM SHARED, a variable named only in a SUB, one named only by
' VARSEG, one named before its first use in the text, strings and fixed-length strings.
DIM a AS INTEGER
x = 1
CONST k = 5
DIM SHARED sh AS LONG
y& = 2
DIM t AS STRING, w AS LONG
DIM fx AS STRING * 6
z% = 3
DIM g AS DOUBLE
DIM fy AS STRING * 1
DIM last AS _BYTE
PRINT "a"; VARPTR(a)
PRINT "x"; VARPTR(x)
PRINT "y&"; VARPTR(y&)
PRINT "t"; VARPTR(t); VARSEG(t)
PRINT "fx"; VARPTR(fx); VARSEG(fx)
PRINT "z%"; VARPTR(z%)
PRINT "g"; VARSEG(g)
PRINT "fy"; VARPTR(fy)
show
PRINT "last"; VARPTR(last)
PRINT "late"; VARPTR(late!)
late! = 4
PRINT k; x; y&; z%; w
' Values through the offsets.
fx = "abcdef"
DEF SEG = VARSEG(fx)
PRINT PEEK(VARPTR(fx)); PEEK(VARPTR(fx) + 5)
POKE VARPTR(fx) + 1, 66
PRINT fx
DEF SEG
SYSTEM

SUB show
    SHARED w AS LONG
    PRINT "sh"; VARPTR(sh); VARSEG(sh)
    PRINT "w"; VARPTR(w)
END SUB
