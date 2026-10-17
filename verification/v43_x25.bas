$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: CHR$ in parentheses then added
CONST z = (CHR$(65)) + "b"
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
