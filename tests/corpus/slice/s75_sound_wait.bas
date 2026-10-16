$CONSOLE:ONLY
' Slice (m3-sound, task 3.1; from verification\v29_b_sound_wait, v29_g_voice): which sound statements hold the
' program until their sound has played. A time is never printed: it is compared with a limit far from the expected
' value, on the side the load of the machine cannot reach ("over" for a statement that holds the program, "under"
' for one that returns at once). All at volume 0: nothing is heard.
DIM SHARED t AS DOUBLE
DIM e AS DOUBLE
CALL start: SOUND 440, 18.2, 0: e = taken#
PRINT "in the foreground, a SOUND of 1 s holds the program (over 0.4 s): "; yn$(e > .4)
drain 0
CALL start: SOUND 440, 1, 0: e = taken#
PRINT "a SOUND of a clock tick does not (under 1 s): "; yn$(e < 1)
CALL start: PLAY "V0 T120 L1 C": e = taken#
PRINT "a PLAY of 2 s holds it (over 1.2 s): "; yn$(e > 1.2)
drain 0
PLAY "MB"
CALL start: PLAY "T120 L1 C": e = taken#
PRINT "after MB the same PLAY returns at once (under 1 s): "; yn$(e < 1); ", with over 1 s left to play: "; yn$(PLAY(0) > 1)
CALL start: SOUND 440, 18.2, 0: e = taken#
PRINT "and so does a SOUND of 1 s (under 1 s): "; yn$(e < 1); ", with over 2 s left: "; yn$(PLAY(0) > 2)
PLAY "MF"
CALL start: SOUND 440, 1, 0: e = taken#
PRINT "after MF a SOUND waits for what is queued (over 1.5 s): "; yn$(e > 1.5)
drain 0
SOUND WAIT
CALL start: SOUND 440, 36.4, 0: e = taken#
PRINT "after SOUND WAIT a SOUND of 2 s returns at once (under 1 s): "; yn$(e < 1); ", with nothing left to play: "; yn$(PLAY(0) = 0)
CALL start: SOUND RESUME: e = taken#
PRINT "SOUND RESUME returns at once (under 1 s): "; yn$(e < 1); ", with over 1 s left: "; yn$(PLAY(0) > 1)
PLAY "MB"
CALL start: SOUND 440, 18.2, 0, , , , 1: e = taken#
PRINT "voice 0 in the background, voice 1 holds the program (over 0.4 s): "; yn$(e > .4)
drain 1
PLAY "", "MB V0"
CALL start: SOUND 440, 36.4, 0, , , , 1: e = taken#
PRINT "after MB in its own string it does not (under 1 s): "; yn$(e < 1); ", with over 1 s left: "; yn$(PLAY(1) > 1)
CALL start: PLAY "", , "V0 T120 L1 C": e = taken#
PRINT "a PLAY with several strings never holds the program (under 1 s): "; yn$(e < 1); ", with over 1 s left in voice 2: "; yn$(PLAY(2) > 1)
SOUND WAIT
SOUND 440, 36.4, 0, , , , 3
PRINT "held back by SOUND WAIT, voice 3 has nothing to play: "; yn$(PLAY(3) = 0)
PLAY "", ""
PRINT "a PLAY with several strings ends the SOUND WAIT (over 1 s left): "; yn$(PLAY(3) > 1)
SYSTEM

SUB start
    t = TIMER(.001)
END SUB

' The seconds since start; a run across midnight is no reason for a "no".
FUNCTION taken#
    DIM e AS DOUBLE
    e = TIMER(.001) - t
    IF e < 0 THEN e = e + 86400
    taken# = e
END FUNCTION

' Waits until a voice has played all it has.
SUB drain (voice AS LONG)
    DO WHILE PLAY(voice) > 0
        _DELAY .05
    LOOP
END SUB

FUNCTION yn$ (b AS LONG)
    IF b THEN yn$ = "yes" ELSE yn$ = "no"
END FUNCTION
