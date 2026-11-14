$CONSOLE:ONLY
d$ = _COMPILEDATE$: t$ = _COMPILETIME$
PRINT "Lengths:"; LEN(d$); LEN(t$)
PRINT "The time's separators: "; MID$(t$, 3, 1); MID$(t$, 6, 1)
END
