$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): which signs the old compiler drops from an
' expression before it reads it. A + right after a + or a - goes; then two - right after an operator go.
ON ERROR GOTO h
DIM a(5), i AS LONG, r AS LONG
DIM i16 AS INTEGER, ub AS _UNSIGNED _BYTE, u64 AS _UNSIGNED _INTEGER64
x = 3: s$ = "a": a(1) = 7: a(3) = 9
i16 = -32768: ub = 200: u64 = 18446744073709551615
PRINT "1."; -+x; 2 + +x; 2 - +x; x + + +x; x - + +x; x + -+x
PRINT "2."; 2 ^ -+x * 3; 2 * -+x; 2 ^ -+5; -+5; x + +5; x - +5
PRINT "3."; 2 - + -x; 2 - + + + -x; -+-+x; 2 * - + x; 2 ^ - + x; 2 ^ - + -x
PRINT "4."; x + + NOT x; x - + NOT x; NOT - + -x; x + +(x); x + +f(1, 2)
PRINT "5. "; "a" + +s$; s$ + +"b"; "a" + - -s$
s$ = "b" + - -s$: PRINT "6. "; s$
x = -+x: PRINT "7."; x: x = 3
PRINT "8."; 1; -+x; a(-+1 + 2); f(1, -+x)
PRINT "9."; x-+x; x++x; x+-+x
PRINT "10."; - -x; 2 * - -x; 2 - -x; 2 - - -x; 2 * - - -x; 2 + - -x; - - -x; - - - -x
PRINT "11."; NOT - -x; 2 > - -x; (- -x); a(- -1); x AND - -x; x = - -x; 2 ^ - -x
PRINT "12."; 2 * - -i16; - -i16; 1 * - -ub; - -ub; 1 * - -u64; - -u64
PRINT "13.";
IF x + +x THEN PRINT " yes";
FOR i = 1 TO 2 + +1 STEP 1: PRINT i;: NEXT
SELECT CASE x: CASE 1 + +2: PRINT " three": END SELECT
PRINT "14. a bound written -+3, DIM run twice:";
FOR r = 1 TO 2
    DIM b(-+3 TO 5)
NEXT
PRINT LBOUND(b)
PRINT "15. a bound written 2 + +3, DIM run twice:";
FOR r = 1 TO 2
    DIM c(2 + +3)
NEXT
PRINT UBOUND(c)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION f (p, q)
    f = p + q
END FUNCTION
