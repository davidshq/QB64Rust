$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): a variable lower bound above a constant upper bound (not
' compared when compiling), with a value that gives two elements and one that gives none.
ON ERROR GOTO h
n = 1
DIM fine(n TO 2, 1 TO 3) AS LONG
PRINT "n = 1:"; LBOUND(fine, 1); UBOUND(fine, 1); UBOUND(fine, 2)
n = 3
PRINT "n = 3:";
DIM none(n TO 2) AS LONG
PRINT LBOUND(none); UBOUND(none)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
