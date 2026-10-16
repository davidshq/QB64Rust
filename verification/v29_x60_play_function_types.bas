$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): one form, on the last line, as code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, t AS STRING, u AS _UNSIGNED LONG
DIM fs AS STRING * 4
PRINT "ok"
SYSTEM
PRINT PLAY(i); PLAY(q); PLAY(s); PLAY(d); PLAY(u)
l = PLAY(1) + 1
IF PLAY(0) > 1 THEN l = 2
