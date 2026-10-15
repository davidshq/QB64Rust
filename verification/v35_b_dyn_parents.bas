$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): _DYNAMIC members of the elements of an array: each element's
' own member, REDIM of the parent (plain, _PRESERVE, _RETAIN), ERASE of the parent, a static parent, SWAP of
' two elements, an element copied to another and to a variable.
TYPE t
    n AS LONG
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM i AS LONG, j AS LONG
DIM w AS t
REDIM p(0 TO 2) AS t
PRINT "1. first bounds of p(1).v:"; LBOUND(p(1).v); UBOUND(p(1).v)
FOR i = 0 TO 2
    p(i).n = i
    REDIM p(i).v(0 TO i + 1) AS LONG
    FOR j = 0 TO i + 1: p(i).v(j) = 100 * i + j: NEXT
NEXT
GOSUB show
PRINT "2. REDIM _PRESERVE p(0 TO 3):"
REDIM _PRESERVE p(0 TO 3) AS t
GOSUB show
PRINT "3. REDIM _RETAIN p(1 TO 3):"
REDIM _RETAIN p(1 TO 3) AS t
GOSUB show
PRINT "4. SWAP p(1), p(2):"
SWAP p(1), p(2)
GOSUB show
PRINT "5. p(3) = p(1), then p(1).v(0) = -1:"
p(3) = p(1)
p(1).v(0) = -1
GOSUB show
PRINT "6. w = p(2), then p(2).v(1) = -2:"
w = p(2)
p(2).v(1) = -2
PRINT "   w:"; w.n; LBOUND(w.v); UBOUND(w.v); w.v(0); w.v(1)
PRINT "7. plain REDIM p(0 TO 1):"
REDIM p(0 TO 1) AS t
GOSUB show
PRINT "8. ERASE p, then REDIM p(0 TO 0):"
ERASE p
REDIM p(0 TO 0) AS t
GOSUB show
PRINT "9. a static parent s(1):"
DIM s(1) AS t
REDIM s(1).v(5 TO 6) AS LONG
s(1).v(6) = 66
PRINT "  "; LBOUND(s(0).v); UBOUND(s(0).v); LBOUND(s(1).v); UBOUND(s(1).v); s(1).v(6)
ERASE s
PRINT "   ERASE s:"; LBOUND(s(1).v); UBOUND(s(1).v); s(1).n
SYSTEM
show:
FOR i = LBOUND(p) TO UBOUND(p)
    PRINT "   p("; i; "): n ="; p(i).n; " v"; LBOUND(p(i).v); "TO"; UBOUND(p(i).v); ":";
    FOR j = LBOUND(p(i).v) TO UBOUND(p(i).v): PRINT p(i).v(j);: NEXT
    PRINT
NEXT
RETURN
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
