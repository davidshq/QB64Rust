$CONSOLE:ONLY
' Verification (m3-sound, task 1.2): the queue read as the slice program will read it. PLAY(0) is read directly
' before and directly after each statement, nothing between the two readings but the statement, and every length
' is a whole number of tenths of a second. All of it at volume 0 and in the background.
DIM SHARED b AS DOUBLE, a AS DOUBLE
PLAY "MB V0"
b = PLAY(0): SOUND 440, 18.2, 0: a = PLAY(0): show "SOUND 440, 18.2, 0"
b = PLAY(0): SOUND 880, 9.1, 0: a = PLAY(0): show "SOUND 880, 9.1, 0"
b = PLAY(0): SOUND 440, 27.3, 0: a = PLAY(0): show "SOUND 440, 27.3, 0"
b = PLAY(0): SOUND 0, 18.2, 0: a = PLAY(0): show "SOUND 0, 18.2, 0"
b = PLAY(0): SOUND 440, 0, 0: a = PLAY(0): show "SOUND 440, 0, 0"
b = PLAY(0): PLAY "T120 L4 C": a = PLAY(0): show "T120 L4 C"
b = PLAY(0): PLAY "C": a = PLAY(0): show "C"
b = PLAY(0): PLAY "T120 L4 C D E": a = PLAY(0): show "T120 L4 C D E"
b = PLAY(0): PLAY "T60 L1 C": a = PLAY(0): show "T60 L1 C"
b = PLAY(0): PLAY "T60 L4 C.": a = PLAY(0): show "T60 L4 C."
b = PLAY(0): PLAY "T60 L8 C.. C16": a = PLAY(0): show "T60 L8 C.. C16"
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
PRINT "PLAY(0) is over 5 at the end: "; PLAY(0) > 5
SYSTEM

SUB show (what AS STRING)
    PRINT what; ":"; CLNG((a - b) * 10)
END SUB
