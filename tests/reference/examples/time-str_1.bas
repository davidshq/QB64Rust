$CONSOLE:ONLY
t$ = TIME$
PRINT "Eight characters, hh:mm:ss:"; LEN(t$); MID$(t$, 3, 1); MID$(t$, 6, 1)
TIME$ = "12:34:56"
unchanged = (LEFT$(TIME$, 5) <> "12:34") OR (LEFT$(t$, 5) = "12:34")
PRINT "TIME$ = changes nothing:"; unchanged
END
