' QB 4.5 (v64_b1-b4): the number 0 as an upper bound under OPTION BASE 1, in a REDIM at run time (the bound in
' a variable and as a literal) under a handler. q45_base1_zero_dim holds the DIM, a compile-time error.
OPTION BASE 1
ON ERROR GOTO h
DIM n AS INTEGER
OPEN "OUT.TXT" FOR OUTPUT AS #1
REDIM x(5) AS INTEGER
PRINT #1, "1. REDIM x(n), n = 0:";
REDIM x(n) AS INTEGER
PRINT #1, ":"; LBOUND(x); UBOUND(x)
PRINT #1, "2. REDIM x(0):";
REDIM x(0) AS INTEGER
PRINT #1, ":"; LBOUND(x); UBOUND(x)
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, " [error"; ERR; "]";
RESUME NEXT
