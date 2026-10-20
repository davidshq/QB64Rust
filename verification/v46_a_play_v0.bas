$CONSOLE:ONLY
' Verification (cws-whole-game, task 2.1): the edit PLAY "V0" at line 218 of Civil War Strategy's copy. After one
' PLAY "V0", the game's own PLAY strings (none holds a V) and its SOUND statements (none gives a volume) run. Does
' each still take its time, and does each queue the length it would? v29_d_volume_zero showed that volume 0 waits
' and queues as the default volume does; here the strings are the game's. Whether a volume holds for the next
' statement, and that nothing is heard, cannot be observed by a program: libqb keeps one amplitude per voice, set
' only by V and by SOUND's third argument (audio.cpp, PSG::SetAmplitude), so the answer is read there.
DIM SHARED last AS DOUBLE
DIM t AS DOUBLE, e AS DOUBLE
PLAY "V0"
PRINT "PLAY(0) after V0 alone:"; PLAY(0)
' Foreground, timed: the short strings and the SOUNDs of the game (lines 2605, 2685, 2692, 1022, 469, 860, 886).
t = TIMER(.001): PLAY "MFMST220o3e4g8g2.g8g8g8o4c2": e = TIMER(.001) - t
PRINT "line 2605 (about 2.2 s) took over 1.8 s: "; yn$(e > 1.8); ", under 3 s: "; yn$(e < 3)
t = TIMER(.001): PLAY "t210l8o4co3bo4l4co3ccL8gfego4co3bo4c": e = TIMER(.001) - t
PRINT "line 2685 (about 2.1 s) took over 1.7 s: "; yn$(e > 1.7); ", under 2.7 s: "; yn$(e < 2.7)
t = TIMER(.001): SOUND 590, .5: SOUND 999, .5: SOUND 1999, .5: e = TIMER(.001) - t
PRINT "line 2692 (3 x 0.5 ticks) took under 0.5 s: "; yn$(e < .5)
t = TIMER(.001): SOUND 77, 9.1: SOUND 59, 9.1: e = TIMER(.001) - t
PRINT "two SOUNDs of 9.1 ticks (1 s) took over 0.7 s: "; yn$(e > .7); ", under 1.5 s: "; yn$(e < 1.5)
t = TIMER(.001): SOUND 2222, 1: SOUND 200, .1: SOUND 50, .1: SOUND 2200, .5: SOUND 2900, .7: e = TIMER(.001) - t
PRINT "lines 469, 860, 886 took under 0.5 s: "; yn$(e < .5)
' Background: the long strings of the title tune and the endings (lines 187-197, 2886-2894, 2901, 2902), queued
' with MB; the game's MF is left out, so nothing waits. Their lengths in tenths of a second.
DO WHILE PLAY(0) > 0: _DELAY .05: LOOP
PLAY "MB"
last = PLAY(0)
PLAY "MST170o1e8o0b8o1e8": show "line 187"
PLAY "e8e4f#8g4f#8": show "line 189"
PLAY "g4e8d2o0b8o1d2 ": show "line 191"
PLAY "o1e8o0b8o1e8e8e4f#8g4f#8g4a8b2g8b2MLg16a16": show "line 193"
PLAY "MSb4b8b8a8g8a4a8a4f#8g4g8MLg8f#8": show "line 195"
PLAY "MSe8f#4f#8f#8g8a8b4.a4.g4.f#4.o0b8o1e8e8e4d8e2.": show "line 197"
PLAY "T130MSO2f16f8.f16f8.e-16d8.f16b-8.o3c16d8.d16d8.c16o2b-4": show "line 2886"
PLAY "o2b-8.a16g8.g16g8.a16b-8.a16b-8.g16f8.g16f8.d16f4": show "line 2888"
PLAY "MNL16o2T120dd.dd.co1b.o2do3g.ab.bb.ag": show "line 2901"
PLAY "MNT160o2L16geL8ccL16cdefL8ggge": show "line 2902"
total = PLAY(0)
t = TIMER(.001)
DO WHILE PLAY(0) > 0: _DELAY .05: LOOP
e = TIMER(.001) - t
PRINT "the queue played out in its time (within 15%): "; yn$(ABS(e - total) < .15 * total)
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
