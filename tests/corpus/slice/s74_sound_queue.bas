$CONSOLE:ONLY
' Slice (m3-sound, task 3.1; from verification\v29_f_queue_slice): what each sound statement adds to PLAY(n), the
' seconds left to play of voice n, in the background. PLAY(n) is read directly before and directly after each
' statement and the difference printed in tenths of a second; every length is a whole number of tenths, so a
' reading may come 50 ms late before a number changes. All at volume 0: nothing is heard. BEEP rings the system's
' bell: it is compiled here and never reached.
DIM SHARED b AS DOUBLE, a AS DOUBLE
DIM t AS STRING, fs AS STRING * 9, i AS INTEGER, l AS LONG, s AS SINGLE, d AS DOUBLE, q AS _INTEGER64
PRINT "at start:"; PLAY(0); PLAY(1); PLAY
IF PLAY(0) > 99 THEN BEEP: CALL BEEP
PLAY "MB V0"
PRINT "SOUND"
b = PLAY(0): SOUND 440, 18.2, 0: a = PLAY(0): show "SOUND 440, 18.2, 0"
b = PLAY(0): SOUND 880, 9.1, 0: a = PLAY(0): show "SOUND 880, 9.1, 0"
b = PLAY(0): SOUND 440, 27.3, 0: a = PLAY(0): show "SOUND 440, 27.3, 0"
b = PLAY(0): SOUND 0, 18.2, 0: a = PLAY(0): show "SOUND 0, 18.2, 0"
b = PLAY(0): SOUND 0, 18.2,: a = PLAY(0): show "SOUND 0, 18.2, with a trailing comma"
b = PLAY(0): SOUND 440, 0, 0: a = PLAY(0): show "SOUND 440, 0, 0"
i = 440: l = 91: s = 9.1: d = 18.2: q = 0
b = PLAY(0): SOUND i, l, q: a = PLAY(0): show "an INTEGER, a LONG of 91 and an _INTEGER64"
b = PLAY(0): SOUND l * 5, s, q: a = PLAY(0): show "a LONG, a SINGLE of 9.1"
b = PLAY(0): SOUND d * 100, d, q, q, i - 436, s - 9.1, q: a = PLAY(0): show "all seven, a DOUBLE of 18.2"
b = PLAY(0): CALL SOUND(440, 18.2, 0): a = PLAY(0): show "CALL SOUND(440, 18.2, 0)"
b = PLAY(0): SOUND 440, 18.2, 0, -1, 3, .5, 0: a = PLAY(0): show "pan -1, waveform 3"
PRINT "PLAY"
b = PLAY(0): PLAY "T120 L4 C": a = PLAY(0): show "T120 L4 C"
b = PLAY(0): PLAY "C": a = PLAY(0): show "C (the tempo and the length are kept)"
b = PLAY(0): PLAY "T120 L4 C D E": a = PLAY(0): show "T120 L4 C D E"
b = PLAY(0): PLAY "T60 L1 C": a = PLAY(0): show "T60 L1 C"
b = PLAY(0): PLAY "T60 L4 C.": a = PLAY(0): show "T60 L4 C."
b = PLAY(0): PLAY "T120 L4 C2": a = PLAY(0): show "T120 L4 C2"
b = PLAY(0): PLAY "T120 P4": a = PLAY(0): show "T120 P4"
b = PLAY(0): PLAY "T120 L4 N0": a = PLAY(0): show "T120 L4 N0"
b = PLAY(0): PLAY "T120 L4 N37": a = PLAY(0): show "T120 L4 N37"
b = PLAY(0): PLAY "MS T120 L4 C": a = PLAY(0): show "MS T120 L4 C"
b = PLAY(0): PLAY "ML T120 L4 C": a = PLAY(0): show "ML T120 L4 C"
b = PLAY(0): PLAY "MN T120 L4 C": a = PLAY(0): show "MN T120 L4 C"
b = PLAY(0): PLAY "t120 l4 o3 c# d- e+ >c <c": a = PLAY(0): show "t120 l4 o3 c# d- e+ >c <c"
b = PLAY(0): PLAY "": a = PLAY(0): show "the empty string"
b = PLAY(0): PLAY "MST150o1e8o0b8o1e8e8": a = PLAY(0): show "MST150o1e8o0b8o1e8e8"
t = "T120 L4 C"
b = PLAY(0): PLAY t: a = PLAY(0): show "a variable"
b = PLAY(0): PLAY t + " " + MID$(t, 9): a = PLAY(0): show "an expression"
b = PLAY(0): PLAY t,: a = PLAY(0): show "a trailing comma"
b = PLAY(0): CALL PLAY(t): a = PLAY(0): show "CALL PLAY(t)"
fs = t
b = PLAY(0): PLAY fs: a = PLAY(0): show "a fixed-length string"
PRINT "voices"
PLAY "", "MB V0", "MB V0", "MB V0"
b = PLAY(1): SOUND 440, 18.2, 0, , , , 1: a = PLAY(1): show "voice 1: SOUND 440, 18.2, 0, , , , 1"
b = PLAY(1): PLAY "T120 L4 C", "T120 L2 C": a = PLAY(1): show "voice 1: the second string, T120 L2 C"
b = PLAY(2): PLAY "", , "T120 L2 C": a = PLAY(2): show "voice 2: the third string"
b = PLAY(3): PLAY "", , , fs: a = PLAY(3): show "voice 3: the fourth string, a fixed-length string"
b = PLAY(3): PLAY "", "T120 L4 C", "T120 L4 C", "T120 L4 C": a = PLAY(3): show "voice 3: of four strings"
PRINT "voice 0 has over 5 s left:"; PLAY(0) > 5; PLAY > 5; " as have the voices 4 and -1:"; PLAY(4) > 5; PLAY(-1) > 5
i = 2: s = 1
PRINT "voices 1 and 2 by a variable:"; PLAY(s) > 1; PLAY(i) > .5
PRINT "SOUND WAIT"
SOUND WAIT
b = PLAY(1): SOUND 440, 18.2, 0, , , , 1: a = PLAY(1): show "held back: SOUND 440, 18.2, 0, , , , 1"
b = PLAY(1): SOUND RESUME: a = PLAY(1): show "SOUND RESUME"
b = PLAY(1): SOUND 440, 18.2, 0, , , , 1 WAIT: a = PLAY(1): show "SOUND 440, 18.2, 0, , , , 1 WAIT"
b = PLAY(1): SOUND 880, 9.1, 0, , , , 1 RESUME: a = PLAY(1): show "SOUND 880, 9.1, 0, , , , 1 RESUME"
SYSTEM

SUB show (what AS STRING)
    PRINT what; ":"; CLNG((a - b) * 10)
END SUB
