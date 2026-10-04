$CONSOLE:ONLY
' INSTR with and without its optional start argument. A start of 0 was put last in case it raised an error (the
' rest of the PRINT skipped, the program stopped); measured 2026-10-03: it does not raise, it behaves like 1.
s$ = "hello world"
PRINT INSTR(s$, "o")
PRINT INSTR(6, s$, "o")
PRINT INSTR(s$, "")
PRINT INSTR(5, s$, "")
PRINT INSTR(20, s$, "o")
PRINT INSTR(2.6, s$, "o")
PRINT INSTR(s$ + "!", "!")
PRINT INSTR("", "")
PRINT INSTR(1, "abc", "abcd")
PRINT "before"; INSTR(0, s$, "o"); "after"
PRINT "not reached if INSTR(0, ...) raises"
END
