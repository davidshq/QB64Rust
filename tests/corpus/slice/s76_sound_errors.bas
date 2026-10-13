$CONSOLE:ONLY
' Slice (m3-sound, task 3.1; from verification\v29_c_sound_errors, v29_h_sound_ranges): which arguments of SOUND and
' which PLAY strings raise error 5. SOUND WAIT comes first, so nothing plays and no statement holds the program, and
' every voice is set to volume 0 before anything is queued, because the PLAY with two strings near the end ends the
' SOUND WAIT. A limit that is accepted is tried with a duration of 0, which queues nothing.
' The first part is checked by libqb before it asks for the sound device, the second after it: on a machine without
' a device the second part raises no error (read in libqb's audio.cpp, PlayWaveform and PlayMML).
DIM SHARED n AS INTEGER
DIM t AS STRING
ON ERROR GOTO handler
SOUND WAIT
SOUND 440, 0, 0: SOUND 440, 0, 0, , , , 1: SOUND 440, 0, 0, , , , 2: SOUND 440, 0, 0, , , , 3
PRINT "checked on any machine"
n = 1: SOUND: PRINT n; "SOUND with no argument"
n = 2: SOUND 0, 1: PRINT n; "SOUND 0, 1"
n = 3: SOUND 19, 1: PRINT n; "SOUND 19, 1"
n = 4: SOUND 20, 0: PRINT n; "SOUND 20, 0"
n = 5: SOUND 32767, 0: PRINT n; "SOUND 32767, 0"
n = 6: SOUND 32768, 1: PRINT n; "SOUND 32768, 1"
n = 7: SOUND -1, 1: PRINT n; "SOUND -1, 1"
n = 8: SOUND 440, -1: PRINT n; "SOUND 440, -1"
n = 9: SOUND 440, 65536: PRINT n; "SOUND 440, 65536"
n = 10: SOUND 440, 0, , , , , 3: PRINT n; "voice 3"
n = 11: SOUND 440, 1, , , , , 4: PRINT n; "voice 4"
n = 12: SOUND 440, 1, , , , , -1: PRINT n; "voice -1"
n = 13: SOUND 10, 1, 2: PRINT n; "a frequency of 10 and a volume of 2: one error"
PRINT "checked with a sound device"
n = 20: SOUND 440, 0, 1: PRINT n; "volume 1"
n = 21: SOUND 440, 0, 0: PRINT n; "volume 0"
n = 22: SOUND 440, 1, 1.5: PRINT n; "volume 1.5"
n = 23: SOUND 440, 1, -.5: PRINT n; "volume -.5"
n = 24: SOUND 440, 0, , 1: PRINT n; "pan 1"
n = 25: SOUND 440, 0, , -1: PRINT n; "pan -1"
n = 26: SOUND 440, 1, , 1.1: PRINT n; "pan 1.1"
n = 27: SOUND 440, 1, , -1.1: PRINT n; "pan -1.1"
n = 28: SOUND 440, 0, , , 1: PRINT n; "waveform 1"
n = 29: SOUND 440, 0, , , 10: PRINT n; "waveform 10"
n = 30: SOUND 440, 1, , , 0: PRINT n; "waveform 0"
n = 31: SOUND 440, 1, , , 11: PRINT n; "waveform 11"
n = 32: SOUND 440, 1, , , -1: PRINT n; "waveform -1"
n = 33: SOUND 440, 0, , , 9, 0: PRINT n; "waveform 9 with the parameter 0"
n = 34: SOUND 440, 0, , , 9, 1: PRINT n; "waveform 9 with the parameter 1"
n = 35: SOUND 440, 1, , , 9, 1.1: PRINT n; "waveform 9 with the parameter 1.1"
n = 36: SOUND 440, 1, , , 9, -.1: PRINT n; "waveform 9 with the parameter -.1"
n = 37: SOUND 440, 0, , , , 2: PRINT n; "the parameter 2 alone (the waveform is 9 from before)"
n = 38: SOUND 440, 0, , , 4, 2: PRINT n; "waveform 4 with the parameter 2"
n = 39: SOUND 440, 1, 2, , , , 3: PRINT n; "a volume of 2 in voice 3"
n = 40: PLAY "H": PRINT n; "PLAY H"
n = 41: PLAY "V0 O6 C": PRINT n; "PLAY O6 C"
n = 42: PLAY "V0 O7 C": PRINT n; "PLAY O7 C"
n = 43: PLAY "V0 T31 C": PRINT n; "PLAY T31 C"
n = 44: PLAY "V0 T256 C": PRINT n; "PLAY T256 C"
n = 45: PLAY "L0 C": PRINT n; "PLAY L0 C"
n = 46: PLAY "L65 C": PRINT n; "PLAY L65 C"
n = 47: PLAY "V0 L64 C": PRINT n; "PLAY L64 C"
n = 48: PLAY "N85": PRINT n; "PLAY N85"
n = 49: PLAY "V0 N84": PRINT n; "PLAY N84"
n = 50: PLAY "V101": PRINT n; "PLAY V101"
n = 51: PLAY "V100 V0": PRINT n; "PLAY V100 V0"
n = 52: PLAY "V0 C D E H C": PRINT n; "PLAY C D E H C"
n = 53: PLAY t: PRINT n; "PLAY of an empty variable"
n = 54: SOUND WAIT: PRINT n; "SOUND WAIT a second time"
n = 55: PRINT n; "nothing to play while held back:"; PLAY(0); PLAY(1); PLAY(2); PLAY(3)
n = 56: PLAY "V0 C", "H": PRINT n; "PLAY with two strings, the second H"
n = 57: SOUND RESUME: PRINT n; "SOUND RESUME"
n = 58: SOUND RESUME: PRINT n; "SOUND RESUME a second time"
PRINT "end"
SYSTEM
handler:
PRINT n; "error"; ERR
RESUME NEXT
