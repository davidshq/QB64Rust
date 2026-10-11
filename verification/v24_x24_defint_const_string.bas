$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a CONST with a string under DEFINT for its letter.
DEFINT A-Z
CONST k = "abc"
PRINT k + "!"
SYSTEM
