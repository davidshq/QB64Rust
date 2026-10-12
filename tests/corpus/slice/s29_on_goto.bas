$CONSOLE:ONLY
' Slice program (m2-core-builtins, D8, D9): ON n GOTO and ON n GOSUB as measured in verification\v20_i_on_goto:
' n = 0, 1, the count, count + 1, 255, 256, 258 (DIVERGENCES-QB45.md Q-001: no error above 255), negative values
' (error 5; the handler prints ERR and resumes next), floats (rounded half to even), an _INTEGER64 beyond LONG (its
' low 32 bits), an error in n (the jump uses the placeholder); ON GOSUB in the main module and in a SUB. No PRINT
' comma.
ON ERROR GOTO h
DIM v(13) AS SINGLE
v(0) = 0: v(1) = 1: v(2) = 3: v(3) = 4: v(4) = 255: v(5) = 256: v(6) = 258: v(7) = -1
v(8) = 1.5: v(9) = 2.5: v(10) = 2.4: v(11) = -0.4: v(12) = 65537: v(13) = -65535
FOR k = 0 TO 13
    PRINT "n ="; v(k); ":";
    ON v(k) GOTO t1, t2, t3
    PRINT " fell through"
    GOTO nxt
t1: PRINT " t1": GOTO nxt
t2: PRINT " t2": GOTO nxt
t3: PRINT " t3"
nxt:
NEXT
DIM q AS _INTEGER64
q = 4294967298
PRINT "_INTEGER64 4294967298:";
ON q GOTO u1, u2, u3
PRINT " fell through"
GOTO un
u1: PRINT " u1": GOTO un
u2: PRINT " u2": GOTO un
u3: PRINT " u3"
un:
DIM i AS INTEGER
i = 2
PRINT "INTEGER 2:";
ON i GOTO w1, w2
PRINT " fell through"
GOTO wn
w1: PRINT " w1": GOTO wn
w2: PRINT " w2"
wn:
PRINT "DOUBLE 2.5:";
ON 2.5# GOTO x1, x2
PRINT " fell through"
GOTO xn
x1: PRINT " x1": GOTO xn
x2: PRINT " x2"
xn:
PRINT "error in n:";
ON ASC("") GOTO e1, e2
PRINT " after ON ASC(empty)"
GOTO en
e1: PRINT " e1": GOTO en
e2: PRINT " e2"
en:
PRINT "error in n, then + 1:";
ON ASC("") + 1 GOTO f1, f2
PRINT " after ON ASC(empty) + 1"
GOTO fn
f1: PRINT " f1": GOTO fn
f2: PRINT " f2"
fn:
ON 258 GOTO z1, z2
PRINT "ON 258 GOTO: next"
GOTO zn
z1: PRINT "z1": GOTO zn
z2: PRINT "z2"
zn:
ON 1 GOSUB g: PRINT "back"
ON 2 GOSUB g, g2
PRINT "back again"
ON 3 GOSUB g, g2
PRINT "back from nothing"
ON -1 GOSUB g, g2
PRINT "after ON -1 GOSUB"
FOR k = 1 TO 2
    ON k GOSUB g, g2
NEXT
PRINT "two GOSUBs from a loop"
sg 1
sg 2
sg 0
SYSTEM

g:
PRINT "in g"
RETURN
g2:
PRINT "in g2"
RETURN

h:
PRINT " [handler"; ERR; "]"
RESUME NEXT

SUB sg (n)
    ON n GOSUB a1, a2
    PRINT "back in sg"; n
    EXIT SUB
a1: PRINT "a1": RETURN
a2: PRINT "a2": RETURN
END SUB
