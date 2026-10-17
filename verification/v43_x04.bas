$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: no argument
CONST z = CHR$()
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
