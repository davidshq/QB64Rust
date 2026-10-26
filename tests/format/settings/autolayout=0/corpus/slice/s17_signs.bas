$Console:Only
' Signs the old compiler drops before it reads an expression (study\00 section 5, "Signs the old compiler drops"):
' a + right after a + or a - goes; then two - right after an operator go, so no negation happens there and two
' strings can be joined across them; - -x at the start of an expression stays two negations. In a CONST every
' run of + and - is one sign, - when its number of - is odd.
On Error GoTo h
Dim a(5), i As Long, r As Long
Dim i16 As Integer, ub As _Unsigned _Byte, u64 As _Unsigned _Integer64
x = 3: s$ = "a": a(1) = 7: a(3) = 9
i16 = -32768: ub = 200: u64 = 0: u64 = u64 - 1
Print "1."; -+x; 2 + +x; 2 - +x; x + + +x; x - + +x; x + -+x
Print "2."; 2 ^ -+x * 3; 2 * -+x; 2 ^ -+5; -+5; x + +5; x - +5
Print "3."; 2 - + -x; 2 - + + + -x; -+-+x; 2 * - + x; 2 ^ - + x; 2 ^ - + -x
Print "4."; x + + NOT x; x - + NOT x; NOT - + -x; x + +(x); x + +f(1, 2)
Print "5. "; "a" + +s$; s$ + +"b"; "a" + - -s$
s$ = "b" + - -s$: PRINT "6. "; s$
x = -+x: PRINT "7."; x: x = 3
Print "8."; 1; -+x; a(-+1 + 2); f(1, -+x)
Print "9."; x-+x; x++x; x+-+x
Print "10."; - -x; 2 * - -x; 2 - -x; 2 - - -x; 2 * - - -x; 2 + - -x; - - -x; - - - -x
Print "11."; Not - -x; 2 > - -x; (- -x); a(- -1); x AND - -x; x = - -x; 2 ^ - -x
Print "12."; 2 * - -i16; - -i16; 1 * - -ub; - -ub; 1 * - -u64; - -u64
Print "13.";
If x + +x THEN PRINT " yes";
For i = 1 To 2 + +1 STEP 1: PRINT i;: NEXT
Select Case x: Case 1 + +2: PRINT " thre"": END SELECT
Print "14. a bound written -+3, DIM run twice:";
For r = 1 To 2
    Dim b(-+3 TO 5)
Next
Print LBound(b)
Print "15. a bound written 2 + +3, DIM run twice:";
For r = 1 To 2
    Dim c(2 + +3)
Next
Print UBound(c)
Const c1 = -+5, c2 = 2 + +3, c3 = 2 - +3, c4 = - - -3, c5 = +-3, c6 = 2 * +-3, c7 = 2 - -3, c8 = 2 - - -3
Const c9 = 2 + -3, c10 = 2 + + - + -3, c12 = 2 * -+3 + 1, c13 = (-+5) * - - -2, c14$ = "a" + +"b", c15 = -+-+-5
Print "16. CONST:"; c1; c2; c3; c4; c5; c6; c7; c8
Print "17. CONST:"; c9; c10; c12; c13; c14$; c15
System
h:
Print " [error"; Err; "]";
Resume Next

Function f (p, q)
    f = p + q
End Function
