$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DEFINT with the underscore as a letter (not last on its line,
' where it would join the next line).
DEFINT _, a
__a = 2.5: a1 = 2.5: PRINT __a; LEN(__a); a1
SYSTEM
