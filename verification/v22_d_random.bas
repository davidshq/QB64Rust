$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "The rest"): RANDOMIZE with each numeric type and USING; RND bare, with
' 1, 0, a negative and a float argument; the result types of RND and TIMER (by their printed digits); TIMER with
' an argument. TIMER's value is only compared.
ON ERROR GOTO h
DIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, uq AS _UNSIGNED _INTEGER64
DIM x AS DOUBLE, t AS DOUBLE
PRINT "no RANDOMIZE:"; RND; RND; RND
RANDOMIZE 5: PRINT "RANDOMIZE 5:"; RND; RND; RND
RANDOMIZE 5: PRINT "RANDOMIZE 5 again:"; RND; RND; RND
RANDOMIZE USING 5: PRINT "RANDOMIZE USING 5:"; RND; RND; RND
RANDOMIZE USING 5: PRINT "RANDOMIZE USING 5 again:"; RND; RND; RND
RANDOMIZE 5: RANDOMIZE 5: PRINT "RANDOMIZE 5 twice:"; RND
b = 5: RANDOMIZE USING b: PRINT "_BYTE 5:"; RND
i = 5: RANDOMIZE USING i: PRINT "INTEGER 5:"; RND
l = 5: RANDOMIZE USING l: PRINT "LONG 5:"; RND
q = 5: RANDOMIZE USING q: PRINT "_INTEGER64 5:"; RND
uq = 5: RANDOMIZE USING uq: PRINT "_UNSIGNED _INTEGER64 5:"; RND
s = 5: RANDOMIZE USING s: PRINT "SINGLE 5:"; RND
d = 5: RANDOMIZE USING d: PRINT "DOUBLE 5:"; RND
f = 5: RANDOMIZE USING f: PRINT "_FLOAT 5:"; RND
RANDOMIZE USING 5.5: PRINT "5.5:"; RND
RANDOMIZE USING -5: PRINT "-5:"; RND
RANDOMIZE USING 0: PRINT "0:"; RND
RANDOMIZE USING 1E+300: PRINT "1E+300:"; RND
q = 5000000000: RANDOMIZE USING q: PRINT "5000000000:"; RND
RANDOMIZE USING l + 1: PRINT "an expression:"; RND
RANDOMIZE USING 5
PRINT "RND(1):"; RND(1); "RND(0) twice:"; RND(0); RND(0); "RND:"; RND
PRINT "RND(-1) twice:"; RND(-1); RND(-1); "then RND:"; RND
PRINT "RND(-2):"; RND(-2); "RND(-1.5):"; RND(-1.5); "RND(2.5):"; RND(2.5)
d = -1: PRINT "RND(d) with -1:"; RND(d): l = -1: PRINT "RND(l) with -1:"; RND(l)
RANDOMIZE USING 5
x = RND: PRINT "RND in a DOUBLE:"; x
x = RND * 1000000000: PRINT "RND * 1000000000:"; x
PRINT "RND + 0#:"; RND + 0#
PRINT "INT(RND * 6) + 1:"; INT(RND * 6) + 1
PRINT "RND in a comparison:"; RND < 1; RND >= 0
t = TIMER: PRINT "TIMER range:"; t >= 0 AND t < 86400
PRINT "TIMER bare twice in order:"; TIMER <= TIMER + 1
t = TIMER(.001): PRINT "TIMER(.001) range:"; t >= 0 AND t < 86400
PRINT "TIMER is whole to a tenth or so:"; ABS(TIMER * 100 - INT(TIMER * 100)) < 100
t = TIMER(0): PRINT "TIMER(0) range:"; t >= 0 AND t < 86400
t = TIMER(1): PRINT "TIMER(1) is whole:"; t = INT(t)
t = TIMER(.5): PRINT "TIMER(.5) is a multiple of a half:"; t * 2 = INT(t * 2)
PRINT "RND(1) parenthesised use:"; (RND) < 1; -RND <= 0
RANDOMIZE TIMER: x = RND: PRINT "RANDOMIZE TIMER:"; x >= 0 AND x < 1
PRINT "raising seed": RANDOMIZE ASC(""): PRINT "after"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
