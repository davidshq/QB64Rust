$CONSOLE:ONLY
s$ = "   both sides   "
PRINT "["; _TRIM$(s$); "]"
t$ = CHR$(9) + "tab" + CHR$(9)
PRINT LEN(_TRIM$(t$))
END
