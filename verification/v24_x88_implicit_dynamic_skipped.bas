$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): below $DYNAMIC, an array used without DIM in a statement that
' GOTO skips.
ON ERROR GOTO h
'$DYNAMIC
GOTO skip
im(3) = 4
skip:
PRINT "UBOUND(im):";
PRINT UBOUND(im)
PRINT "im(3) = 5:";
im(3) = 5
PRINT im(3)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
