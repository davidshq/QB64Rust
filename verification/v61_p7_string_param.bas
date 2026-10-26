$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of a string parameter called with string
' expressions: the copy the SUB makes is not in conventional memory (its SADD is a heap address cut to 16 bits,
' not printed).
t "ab" + "c"
t ("xyz")
SYSTEM

SUB t (s$)
    PRINT VARPTR(s$); VARSEG(s$); LEN(s$)
END SUB
