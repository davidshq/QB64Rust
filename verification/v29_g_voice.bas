$CONSOLE:ONLY
' Verification (m3-sound, task 1.2): a voice of its own. Voice 0 is put in the background; is voice 1 then still in
' the foreground, and how is it put in the background? All at volume 0.
DIM t AS DOUBLE, e AS DOUBLE, b AS DOUBLE, a AS DOUBLE
PLAY "MB V0"
PRINT "at start: PLAY(0)"; PLAY(0); "PLAY(1)"; PLAY(1)
b = PLAY(0)
t = TIMER(.001): SOUND 440, 18.2, 0, , , , 1: e = TIMER(.001) - t
a = PLAY(0)
PRINT "voice 0 in the background, SOUND 440, 18.2, 0, , , , 1 took over 0.4 s: "; yn$(e > .4); ", under 1.5 s: "; yn$(e < 1.5)
PRINT "it added to voice 0, in tenths:"; CLNG((a - b) * 10)
DO WHILE PLAY(1) > 0: _DELAY .05: LOOP
b = PLAY(1)
t = TIMER(.001): SOUND 440, 1, 0, , , , 1: e = TIMER(.001) - t
PRINT "SOUND 440, 1, 0, , , , 1 took under 0.3 s: "; yn$(e < .3)
DO WHILE PLAY(1) > 0: _DELAY .05: LOOP
PLAY "", "MB V0"
b = PLAY(1)
t = TIMER(.001): SOUND 440, 18.2, 0, , , , 1: e = TIMER(.001) - t
a = PLAY(1)
PRINT "after PLAY with MB as its second string, the same took under 0.3 s: "; yn$(e < .3)
PRINT "it added to voice 1, in tenths:"; CLNG((a - b) * 10)
b = PLAY(1)
PLAY "", "T120 L4 C"
a = PLAY(1)
PRINT "PLAY with T120 L4 C as its second string added to voice 1, in tenths:"; CLNG((a - b) * 10)
b = PLAY(2)
t = TIMER(.001): PLAY "", , "V0 T120 L1 C": e = TIMER(.001) - t
a = PLAY(2)
PRINT "voice 2 in the foreground, PLAY with V0 T120 L1 C as its third string took over 1.2 s: "; yn$(e > 1.2)
b = PLAY(0): a = PLAY(3)
PLAY "T120 L4 C", "T120 L4 C", , "MB V0 T120 L2 C"
PRINT "four strings, the third left out: voice 0 over 0.3: "; yn$(PLAY(0) - b > .3); ", voice 3 over 0.8: "; yn$(PLAY(3) - a > .8)
SYSTEM

FUNCTION yn$ (b AS LONG)
    IF b THEN yn$ = "yes" ELSE yn$ = "no"
END FUNCTION
