$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: two arguments
CONST z = CHR$(1, 2)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
