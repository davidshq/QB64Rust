$CONSOLE:ONLY
' Slice (m4-memory-records, task 2.4): variables in conventional memory: VARPTR, VARSEG, SADD and VARPTR$ of
' numbers, strings, a fixed-length string, a member and elements, with PEEK and POKE through them
' (verification\v56_a1, a4, b1, t01).
TYPE t
    a AS LONG
    v AS _BYTE
END TYPE
DIM i AS INTEGER, s AS STRING, fx AS STRING * 4, r AS t, sa(1 TO 2) AS STRING, k AS INTEGER
PRINT VARPTR(i); VARSEG(i); VARPTR(s); VARPTR(fx); VARPTR(r.v); VARPTR(sa(2))
s = "hello"
PRINT SADD(s); LEN(s); SADD(sa(1))
DEF SEG = VARSEG(s)
PRINT PEEK(VARPTR(s)); PEEK(SADD(s)); PEEK(SADD(s) + 4)
i = 258: POKE VARPTR(i), 3
fx = "abcd": POKE VARPTR(fx) + 1, 66
DEF SEG
PRINT i; fx
v$ = VARPTR$(i) + VARPTR$(s) + VARPTR$(fx) + VARPTR$(r.v) + VARPTR$(sa(1))
FOR k = 1 TO LEN(v$): PRINT ASC(v$, k);: NEXT: PRINT
SWAP s, sa(1)
PRINT VARPTR(s); SADD(s); "["; s; "]["; sa(1); "]"
PRINT HEX$(VARPTR(i)); " "; OCT$(VARSEG(fx)); " "; _BIN$(SADD(sa(1)))
SYSTEM
