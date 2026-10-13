$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): what PLAY(n) gives for sound that is queued in the background (PLAY "MB").
' Each line prints what its statement added to the seconds left to play of voice 0, in tenths of a second: the
' sound plays while the program runs, so a finer number would differ from run to run. BEEP runs once at the end.
DIM SHARED last AS DOUBLE
PRINT "at start"; PLAY(0)
PLAY "MB"
PRINT "after MB"; PLAY(0)
SOUND 440, 18.2: show "SOUND 440, 18.2"
SOUND 880, 9.1: show "SOUND 880, 9.1"
SOUND 440, 1: show "SOUND 440, 1"
SOUND 440, .03: show "SOUND 440, .03"
SOUND 0, 18.2: show "SOUND 0, 18.2"
SOUND 440, 0: show "SOUND 440, 0"
SOUND 440, 18.2, .5, -1, 4: show "SOUND 440, 18.2, .5, -1, 4"
PLAY "T120 L4 C": show "T120 L4 C"
PLAY "C": show "C (the tempo and length are kept)"
PLAY "T120 L4 C D E": show "T120 L4 C D E"
PLAY "T60 L1 C": show "T60 L1 C"
PLAY "T120 L4 C.": show "T120 L4 C."
PLAY "T120 L4 C..": show "T120 L4 C.."
PLAY "T120 C8": show "T120 C8"
PLAY "T120 P4": show "T120 P4"
PLAY "T120 L4 N0": show "T120 L4 N0"
PLAY "T120 L4 N37": show "T120 L4 N37"
PLAY "MS T120 L4 C": show "MS T120 L4 C"
PLAY "ML T120 L4 C": show "ML T120 L4 C"
PLAY "MN T120 L4 C": show "MN T120 L4 C"
PLAY "MB T120 L4 C": show "MB T120 L4 C"
PLAY "t120 l4 o3 c# d- e+ >c <c": show "t120 l4 o3 c# d- e+ >c <c"
PLAY "": show "the empty string"
PLAY "   ": show "blanks"
PLAY "MST170o1e8o0b8o1e8": show "MST170o1e8o0b8o1e8 (the game)"
PRINT "PLAY(1)"; PLAY(1); "PLAY(3)"; PLAY(3)
PRINT "PLAY(4) = PLAY(0):"; PLAY(4) = PLAY(0); " PLAY(-1) = PLAY(0):"; PLAY(-1) = PLAY(0)
PRINT "PLAY = PLAY(0):"; PLAY = PLAY(0)
SOUND 440, 18.2, , , , , 1
PRINT "voice 1 after SOUND 440, 18.2, , , , , 1:"; CLNG(PLAY(1) * 10)
BEEP
PRINT "after BEEP"
SYSTEM

SUB show (what AS STRING)
    DIM now AS DOUBLE
    now = PLAY(0)
    PRINT what; ":"; CLNG((now - last) * 10)
    last = now
END SUB
