' TEST: typed
' The screen and keyboard functions in the typed tree (m3-window-output tasks 3.3 and 4.1, design D2 and D5;
' measured from the old compiler's C++ and compared by tests\callsite). INKEY$ is a string and CSRLIN a LONG, both
' called by their bare name; POS has one LONG slot and is a LONG; SCREEN has two LONG slots and an optional third
' and is an _UNSIGNED LONG (its negation an _INTEGER64); POINT has a slot and an optional second, each taking a
' number of any type as it is, and is a DOUBLE (stored into an integer it is rounded). INKEY without its $ is a
' variable
DIM l AS LONG, i AS INTEGER, q AS _INTEGER64, sg AS SINGLE, d AS DOUBLE, f AS _FLOAT, u AS _UNSIGNED LONG
DIM s AS STRING
s = INKEY$
s = INKEY$ + "x"
l = LEN(INKEY$)
IF INKEY$ = "" THEN l = 1
l = CSRLIN
i = CSRLIN
d = CSRLIN
l = CSRLIN + 1
q = CSRLIN * 4294967296
d = CSRLIN / 3
l = -CSRLIN
s = HEX$(CSRLIN)
l = POS(0)
l = POS(i)
l = POS(q)
l = POS(sg)
l = POS(1.5)
d = POS(0) / 3
q = POS(0) * 100000
s = HEX$(POS(0))
u = SCREEN(1, 2)
l = SCREEN(1, 2)
u = SCREEN(1, 2, 1)
u = SCREEN(i, q)
u = SCREEN(sg, d, 0.5)
q = SCREEN(1, 2) + 1
d = SCREEN(1, 2) / 3
q = -SCREEN(1, 2)
d = SCREEN(1, 2) - 4294967295
s = HEX$(SCREEN(1, 2))
d = POINT(1, 2)
d = POINT(0)
l = POINT(1, 2)
u = POINT(1, 2)
d = POINT(i, q)
d = POINT(sg, d)
d = POINT(f, 1.5)
d = POINT(u)
d = POINT(1, 2) / 3
q = POINT(1, 2) + 1
s = HEX$(POINT(1, 2))
l = INKEY
PRINT INKEY$; CSRLIN; POS(0); SCREEN(1, 1); POINT(1, 1)
SYSTEM
