$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 5.1): an array used without DIM above the OPTION _EXPLICITARRAY line.
z(1) = 5
PRINT z(1)
OPTION _EXPLICITARRAY
SYSTEM
