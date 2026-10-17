$CONSOLE:ONLY
' Verification (m3-errorline-chr-const, before its proposal): CHR$ in a CONST, one form: a string constant as the argument
CONST s = "A": CONST z = CHR$(s)
PRINT "accepted:"; LEN(z); ASC(z + " ")
SYSTEM
