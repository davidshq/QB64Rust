' TEST: typed
$CONSOLE:ONLY
' The function PLAY (m3-sound task 2.1, design D1; verification\v29_x55, x56, x60, x64): a DOUBLE, with one optional
' slot of any numeric type, so it is also called bare. ON PLAY(n) GOSUB is ON <number> GOSUB with the function, not
' QuickBASIC's event, as in the old compiler
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT, t AS STRING, u AS _UNSIGNED LONG
d = PLAY(0)
d = PLAY
PRINT PLAY(i); PLAY(l); PLAY(q); PLAY(s); PLAY(d); PLAY(f); PLAY(u)
l = PLAY(1) + 1
IF PLAY(0) > 1 THEN l = 2
s = PLAY
t = STR$(PLAY(0))
PRINT PLAY(4) = PLAY(0); PLAY = PLAY(0)
ON PLAY(5) GOSUB done
ON PLAY GOSUB done, done
SYSTEM
done:
RETURN
