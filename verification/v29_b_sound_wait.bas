$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): which sound statements hold the program until their sound has played.
' Times are printed as yes or no against wide limits, so the recording does not depend on the load.
DIM t AS DOUBLE, e AS DOUBLE
t = TIMER(.001): SOUND 440, 18.2: e = TIMER(.001) - t
PRINT "SOUND 440, 18.2 took over 0.4 s: "; yn$(e > .4); ", under 1.5 s: "; yn$(e < 1.5)
t = TIMER(.001): SOUND 440, 1: e = TIMER(.001) - t
PRINT "SOUND 440, 1 took under 0.3 s: "; yn$(e < .3)
t = TIMER(.001): PLAY "T120 L1 C": e = TIMER(.001) - t
PRINT "PLAY T120 L1 C (2 s) took over 1.2 s: "; yn$(e > 1.2); ", under 2.5 s: "; yn$(e < 2.5)
PLAY "MB"
t = TIMER(.001): PLAY "T120 L1 C": e = TIMER(.001) - t
PRINT "after MB, PLAY T120 L1 C took under 0.3 s: "; yn$(e < .3); ", PLAY(0) over 1.5: "; yn$(PLAY(0) > 1.5)
t = TIMER(.001): SOUND 440, 18.2: e = TIMER(.001) - t
PRINT "after MB, SOUND 440, 18.2 took under 0.3 s: "; yn$(e < .3); ", PLAY(0) over 2.5: "; yn$(PLAY(0) > 2.5)
PLAY "MF"
t = TIMER(.001): SOUND 440, 1: e = TIMER(.001) - t
PRINT "after MF, with 3 s queued, SOUND 440, 1 took over 1.5 s: "; yn$(e > 1.5)
SOUND WAIT
t = TIMER(.001): SOUND 440, 18.2: e = TIMER(.001) - t
PRINT "after SOUND WAIT, SOUND 440, 18.2 took under 0.3 s: "; yn$(e < .3)
t = TIMER(.001): SOUND RESUME: e = TIMER(.001) - t
PRINT "SOUND RESUME took under 0.3 s: "; yn$(e < .3); ", PLAY(0) over 0.5: "; yn$(PLAY(0) > .5)
t = TIMER(.001)
SYSTEM

FUNCTION yn$ (b AS LONG)
    IF b THEN yn$ = "yes" ELSE yn$ = "no"
END FUNCTION
