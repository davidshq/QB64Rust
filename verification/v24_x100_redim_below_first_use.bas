$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array used above the REDIM that declares it, reached by GOTO.
GOTO make
back:
r(1) = 5: PRINT r(1); UBOUND(r)
SYSTEM
make:
REDIM r(3) AS LONG
GOTO back
SYSTEM
