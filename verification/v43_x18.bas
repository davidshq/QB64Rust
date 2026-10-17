$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a minus before CHR$
CONST z = -CHR$(65)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
