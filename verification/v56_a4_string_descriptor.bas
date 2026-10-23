$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): a string named by VARPTR keeps its descriptor in conventional
' memory: VARPTR, SADD and the descriptor's length and offset (read with PEEK) after each kind of assignment.
DIM SHARED t AS STRING
DIM u AS STRING, v AS STRING, n AS INTEGER
DATA "from data"
PRINT "start"; VARPTR(t); SADD(t); LEN(t)
desc
t = "abc"
desc
t = t + STRING$(300, "x")
desc
t = LEFT$(t, 2)
desc
t = UCASE$(t)
desc
u = "a string not in conventional memory"
t = u
desc
v = "zz"
SWAP t, v
desc
PRINT v
MID$(t, 1, 1) = "Q"
desc
grow t
desc
READ t
desc
t = ""
desc
t = SPACE$(40)
LSET t = "left"
desc
n = 5
t = STR$(n)
desc
SYSTEM

SUB desc
    DEF SEG = VARSEG(t)
    l = PEEK(VARPTR(t)) + 256 * PEEK(VARPTR(t) + 1)
    o = PEEK(VARPTR(t) + 2) + 256 * PEEK(VARPTR(t) + 3)
    DEF SEG
    PRINT VARPTR(t); SADD(t); LEN(t); l; o; "["; LEFT$(t, 12); "]"
END SUB

SUB grow (s AS STRING)
    s = s + "-grown in a SUB-"
END SUB
