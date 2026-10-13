$CONSOLE:ONLY
' Verification (m3-sound, task 1.2): sound at volume 0. Do the waits of v29_b_sound_wait and the queue lengths of
' v29_a_sound_queue stay the same when nothing is heard? Volume 0 is set by SOUND's third argument and by PLAY "V0".
' Whether a volume holds for the next statement cannot be observed by a program; it is read in libqb's source.
DIM SHARED last AS DOUBLE
DIM t AS DOUBLE, e AS DOUBLE
t = TIMER(.001): SOUND 440, 18.2, 0: e = TIMER(.001) - t
PRINT "SOUND 440, 18.2, 0 took over 0.4 s: "; yn$(e > .4); ", under 1.5 s: "; yn$(e < 1.5)
t = TIMER(.001): SOUND 440, 1, 0: e = TIMER(.001) - t
PRINT "SOUND 440, 1, 0 took under 0.3 s: "; yn$(e < .3)
t = TIMER(.001): PLAY "V0 T120 L1 C": e = TIMER(.001) - t
PRINT "PLAY V0 T120 L1 C (2 s) took over 1.2 s: "; yn$(e > 1.2); ", under 2.5 s: "; yn$(e < 2.5)
t = TIMER(.001): PLAY "T120 L1 C": e = TIMER(.001) - t
PRINT "PLAY T120 L1 C after it took over 1.2 s: "; yn$(e > 1.2); ", under 2.5 s: "; yn$(e < 2.5)
DO WHILE PLAY(0) > 0: _DELAY .05: LOOP
PLAY "MB"
last = PLAY(0)
SOUND 440, 18.2, 0: show "SOUND 440, 18.2, 0"
SOUND 880, 9.1, 0: show "SOUND 880, 9.1, 0"
SOUND 440, 18.2: show "SOUND 440, 18.2 (no volume given)"
SOUND 0, 18.2, 0: show "SOUND 0, 18.2, 0"
PLAY "V0 T120 L4 C": show "V0 T120 L4 C"
PLAY "C": show "C"
PLAY "V0 T120 L4 C D E": show "V0 T120 L4 C D E"
PLAY "V0 T60 L1 C": show "V0 T60 L1 C"
PLAY "V0 T120 P4": show "V0 T120 P4"
PLAY "V0": show "V0 alone"
t = TIMER(.001): PLAY "T120 L1 C": e = TIMER(.001) - t
PRINT "in the background, PLAY T120 L1 C took under 0.3 s: "; yn$(e < .3)
SYSTEM

SUB show (what AS STRING)
    DIM now AS DOUBLE
    now = PLAY(0)
    PRINT what; ":"; CLNG((now - last) * 10)
    last = now
END SUB

FUNCTION yn$ (b AS LONG)
    IF b THEN yn$ = "yes" ELSE yn$ = "no"
END FUNCTION
