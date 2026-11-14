$CONSOLE:ONLY
o$ = _OS$
PRINT "In brackets:"; LEFT$(o$, 1) = "["; RIGHT$(o$, 1) = "]"
PRINT "Names the bits:"; INSTR(o$, "[32BIT]") > 0 OR INSTR(o$, "[64BIT]") > 0
END
