$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: an argument of 1E300
CONST z = CHR$(1E300)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
