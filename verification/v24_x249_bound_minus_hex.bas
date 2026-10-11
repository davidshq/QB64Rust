$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): a minus before a hexadecimal number as a bound: one number (a
' static array, REDIM raises 10) or an operator (a dynamic one, REDIM gives 20)?
ON ERROR GOTO h
DIM hx(-&H5 TO 5) AS LONG
PRINT "-&H5 TO 5:"; LBOUND(hx);
REDIM hx(20) AS LONG
PRINT UBOUND(hx)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
