$CONSOLE:ONLY
' RANDOMIZE, RND and TIMER (spec language/builtin-statements "RANDOMIZE"; language/builtin-functions "Built-ins
' without parentheses"): the sequence without RANDOMIZE; RANDOMIZE n, which depends on what was drawn before, and
' RANDOMIZE USING n, which does not; a seed of every numeric type, a float, a negative one; RND bare, with 1, 0 and
' negative arguments; RND is SINGLE in arithmetic; TIMER bare and with an argument, only in comparisons; a raising
' seed.
ON ERROR GOTO h
DIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT
DIM uq AS _UNSIGNED _INTEGER64, x AS DOUBLE, t AS DOUBLE
PRINT "no RANDOMIZE:"; RND; RND; RND
RANDOMIZE 5: PRINT "RANDOMIZE 5:"; RND; RND; RND
RANDOMIZE 5: PRINT "RANDOMIZE 5 again:"; RND; RND; RND
RANDOMIZE USING 5: PRINT "RANDOMIZE USING 5:"; RND; RND; RND
RANDOMIZE USING 5: PRINT "RANDOMIZE USING 5 again:"; RND; RND; RND
RANDOMIZE (5): PRINT "RANDOMIZE (5):"; RND
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
PRINT "RND in a comparison:"; RND < 1; RND >= 0; (RND) < 1; -RND <= 0
t = TIMER: PRINT "TIMER range:"; t >= 0 AND t < 86400
PRINT "TIMER twice in order:"; TIMER <= TIMER + 1
t = TIMER(.001): PRINT "TIMER(.001) range:"; t >= 0 AND t < 86400
PRINT "TIMER(0)": t = TIMER(0): PRINT t
t = TIMER(1): PRINT "TIMER(1) is whole:"; t = INT(t)
t = TIMER(.5): PRINT "TIMER(.5) is a multiple of a half:"; t * 2 = INT(t * 2)
l = 1: t = TIMER(l): PRINT "TIMER(l) is whole:"; t = INT(t)
RANDOMIZE TIMER: x = RND: PRINT "RANDOMIZE TIMER:"; x >= 0 AND x < 1
PRINT "raising seed": RANDOMIZE ASC(""): PRINT "after"
PRINT "in a FUNCTION:"; dice&
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT

FUNCTION dice&
    RANDOMIZE USING 7
    dice& = INT(RND * 6) + 1
END FUNCTION
