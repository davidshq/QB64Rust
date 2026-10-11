$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.1): a second DIM with constant bounds of a static array, below
' $DYNAMIC.
ON ERROR GOTO h
DIM st(5) AS LONG
st(1) = 3
'$DYNAMIC
PRINT "second DIM:";
DIM st(5) AS LONG
PRINT UBOUND(st); st(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
