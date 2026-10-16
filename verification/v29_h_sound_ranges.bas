$CONSOLE:ONLY
' Verification (m3-sound, task 1.2): the ranges v29_c_sound_errors left open: the waveform parameter, the pan
' position, the volume, the voice, the waveform, and the volume of a PLAY string. SOUND WAIT comes first, so nothing
' plays and no statement holds the program.
DIM SHARED n AS INTEGER
ON ERROR GOTO handler
SOUND WAIT
n = 1: SOUND 440, 1, , , 9, 0: PRINT n; "SOUND 440, 1, , , 9, 0"
n = 2: SOUND 440, 1, , , 9, 1: PRINT n; "SOUND 440, 1, , , 9, 1"
n = 3: SOUND 440, 1, , , 9, 1.1: PRINT n; "SOUND 440, 1, , , 9, 1.1"
n = 4: SOUND 440, 1, , , 9, -.1: PRINT n; "SOUND 440, 1, , , 9, -.1"
n = 5: SOUND 440, 1, , , , 2: PRINT n; "SOUND 440, 1, , , , 2 (the waveform is 9 from before)"
n = 6: SOUND 440, 1, , , 4, 2: PRINT n; "SOUND 440, 1, , , 4, 2"
n = 7: SOUND 440, 1, , , 4, -2: PRINT n; "SOUND 440, 1, , , 4, -2"
n = 8: SOUND 440, 1, , -1: PRINT n; "SOUND 440, 1, , -1"
n = 9: SOUND 440, 1, , -1.1: PRINT n; "SOUND 440, 1, , -1.1"
n = 10: SOUND 440, 1, , 1.1: PRINT n; "SOUND 440, 1, , 1.1"
n = 11: SOUND 440, 1, 0: PRINT n; "SOUND 440, 1, 0"
n = 12: SOUND 440, 1, 1.1: PRINT n; "SOUND 440, 1, 1.1"
n = 13: SOUND 440, 1, , , -1: PRINT n; "SOUND 440, 1, , , -1"
n = 14: SOUND 440, 1, , , , , 0: PRINT n; "SOUND 440, 1, , , , , 0"
n = 15: SOUND 440, 1, , , , , -1: PRINT n; "SOUND 440, 1, , , , , -1"
n = 16: SOUND 440, 1, 2, , , , 4: PRINT n; "SOUND 440, 1, 2, , , , 4 (a volume and a voice out of range)"
n = 17: SOUND 10, 1, 2: PRINT n; "SOUND 10, 1, 2 (a frequency and a volume out of range)"
n = 18: SOUND 440.5, 1.5: PRINT n; "SOUND 440.5, 1.5"
n = 19: PLAY "V0": PRINT n; "PLAY V0"
n = 20: PLAY "V100": PRINT n; "PLAY V100"
n = 21: PLAY "V101": PRINT n; "PLAY V101"
n = 22: PLAY "H": PRINT n; "PLAY H"
n = 23: SOUND 440, 18.2, 0, , , , 3: PRINT n; "SOUND 440, 18.2, 0, , , , 3"
n = 24: PRINT n; "PLAY(0) is 0 before a PLAY with two strings: "; PLAY(0) = 0
PLAY "", ""
n = 25: PRINT n; "and over 0 after it (it ends the SOUND WAIT): "; PLAY(0) > 0; " PLAY(3): "; PLAY(3) > 0
PRINT "end"
SYSTEM
handler:
PRINT n; "error"; ERR; "in line"; _ERRORLINE
RESUME NEXT
