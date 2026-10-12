' TEST: typed
$CONSOLE:ONLY
' RND, TIMER and RANDOMIZE in the typed tree (m2-builtin-statements task 5.4; measured in verification\v22_d_random
' and from the C++): RND is SINGLE, with its optional argument passed in its own type; TIMER is believed SINGLE and
' held DOUBLE, its optional argument converted to DOUBLE; both are called by their bare names; RANDOMIZE takes a
' DOUBLE seed, with or without USING
DIM d AS DOUBLE, l AS LONG, s AS SINGLE
d = RND
d = RND(1)
d = RND(l)
d = RND(d)
s = RND * 2
d = TIMER
d = TIMER(.001)
d = TIMER(l)
s = TIMER + 1
PRINT RND; TIMER
RANDOMIZE 5
RANDOMIZE USING l
RANDOMIZE TIMER
RANDOMIZE (5)
