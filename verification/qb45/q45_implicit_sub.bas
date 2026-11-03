' QB 4.5 (DIVERGENCES-QB45.md Q-009): an array used without DIM inside a SUB, a FUNCTION and
' a SUB ... STATIC, each called twice. QB64pe raises error 9 on every use in the first two;
' in the STATIC SUB it is an array 0 to 10 kept between calls.
DECLARE SUB s ()
DECLARE FUNCTION f! ()
DECLARE SUB st ()
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
CALL s
CALL s
PRINT #1, "f:"; f
PRINT #1, "f:"; f
CALL st
CALL st
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]";
RESUME NEXT

FUNCTION f
y(3) = y(3) + 2
f = y(3) + UBOUND(y)
END FUNCTION

SUB s
PRINT #1, "s:"; x(2);
x(2) = x(2) + 5
PRINT #1, x(2); LBOUND(x); UBOUND(x);
x(11) = 1
PRINT #1,
END SUB

SUB st STATIC
PRINT #1, "st:"; w(2);
w(2) = w(2) + 5
PRINT #1, w(2); LBOUND(w); UBOUND(w);
w(11) = 1
PRINT #1,
END SUB
