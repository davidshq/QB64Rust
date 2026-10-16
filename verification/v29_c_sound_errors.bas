$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): which arguments of SOUND and PLAY are a run-time error. SOUND WAIT comes
' first, so nothing plays and no statement holds the program.
DIM SHARED n AS INTEGER
DIM t AS STRING
ON ERROR GOTO handler
SOUND WAIT
n = 1: SOUND 0, 1: PRINT n; "SOUND 0, 1"
n = 2: SOUND 19, 1: PRINT n; "SOUND 19, 1"
n = 3: SOUND 20, 1: PRINT n; "SOUND 20, 1"
n = 4: SOUND 36, 1: PRINT n; "SOUND 36, 1"
n = 5: SOUND 37, 1: PRINT n; "SOUND 37, 1"
n = 6: SOUND 32767, 1: PRINT n; "SOUND 32767, 1"
n = 7: SOUND 32768, 1: PRINT n; "SOUND 32768, 1"
n = 8: SOUND -1, 1: PRINT n; "SOUND -1, 1"
n = 9: SOUND 440, -1: PRINT n; "SOUND 440, -1"
n = 10: SOUND 440, 0: PRINT n; "SOUND 440, 0"
n = 11: SOUND 440, 65535: PRINT n; "SOUND 440, 65535"
n = 12: SOUND 440, 65536: PRINT n; "SOUND 440, 65536"
n = 13: SOUND 440, 1, 1: PRINT n; "SOUND 440, 1, 1"
n = 14: SOUND 440, 1, 1.5: PRINT n; "SOUND 440, 1, 1.5"
n = 15: SOUND 440, 1, -.5: PRINT n; "SOUND 440, 1, -.5"
n = 16: SOUND 440, 1, , 1: PRINT n; "SOUND 440, 1, , 1"
n = 17: SOUND 440, 1, , 2: PRINT n; "SOUND 440, 1, , 2"
n = 18: SOUND 440, 1, , , 0: PRINT n; "SOUND 440, 1, , , 0"
n = 19: SOUND 440, 1, , , 1: PRINT n; "SOUND 440, 1, , , 1"
n = 20: SOUND 440, 1, , , 10: PRINT n; "SOUND 440, 1, , , 10"
n = 21: SOUND 440, 1, , , 11: PRINT n; "SOUND 440, 1, , , 11"
n = 22: SOUND 440, 1, , , 9, 2: PRINT n; "SOUND 440, 1, , , 9, 2"
n = 23: SOUND 440, 1, , , , , 3: PRINT n; "SOUND 440, 1, , , , , 3"
n = 24: SOUND 440, 1, , , , , 4: PRINT n; "SOUND 440, 1, , , , , 4"
n = 25: SOUND: PRINT n; "SOUND"
n = 26: PLAY "H": PRINT n; "PLAY H"
n = 27: PLAY "O7 C": PRINT n; "PLAY O7 C"
n = 28: PLAY "O6 C": PRINT n; "PLAY O6 C"
n = 29: PLAY "T31 C": PRINT n; "PLAY T31 C"
n = 30: PLAY "T256 C": PRINT n; "PLAY T256 C"
n = 31: PLAY "L0 C": PRINT n; "PLAY L0 C"
n = 32: PLAY "L65 C": PRINT n; "PLAY L65 C"
n = 33: PLAY "N85": PRINT n; "PLAY N85"
n = 34: PLAY "C D E H C": PRINT n; "PLAY C D E H C"
n = 35: PLAY t: PRINT n; "PLAY of an empty variable"
n = 36: PLAY "C", "H": PRINT n; "PLAY C, H"
n = 37: SOUND WAIT: PRINT n; "SOUND WAIT a second time"
n = 38: SOUND RESUME: PRINT n; "SOUND RESUME"
n = 39: SOUND RESUME: PRINT n; "SOUND RESUME a second time"
PRINT "end"
SYSTEM
handler:
PRINT n; "error"; ERR; "in line"; _ERRORLINE
RESUME NEXT
