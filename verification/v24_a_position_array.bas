$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array declared without a type before a DEFxxx, named after
' it.
ON ERROR GOTO h
DIM t(3)
t(1) = 2.5
DEFINT A-Z
PRINT "DIM t(3), t(1) = 2.5, then DEFINT: t(1), t!(1), UBOUND(t), UBOUND(t!)"; t(1); t!(1); UBOUND(t); UBOUND(t!)
t(1) = 7.5
PRINT "after t(1) = 7.5: t(1), t!(1), t%(1)"; t(1); t!(1); t%(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
