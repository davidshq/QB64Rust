$Console:Only
' Slice (m3-sound, task 3.1; from verification\v29_c_sound_errors, v29_h_sound_ranges): which arguments of SOUND and
' which PLAY strings raise error 5. SOUND WAIT comes first, so nothing plays and no statement holds the program, and
' every voice is set to volume 0 before anything is queued, because the PLAY with two strings near the end ends the
' SOUND WAIT. A limit that is accepted is tried with a duration of 0, which queues nothing.
' The first part is checked by libqb before it asks for the sound device, the second after it: on a machine without
' a device the second part raises no error (read in libqb's audio.cpp, PlayWaveform and PlayMML).
Dim Shared n As Integer
Dim t As String
On Error GoTo handler
Sound Wait
Sound 440, 0, 0: Sound 440, 0, 0, , , , 1: Sound 440, 0, 0, , , , 2: Sound 440, 0, 0, , , , 3
Print "checked on any machine"
n = 1: Sound: Print n; "SOUND with no argument"
n = 2: Sound 0, 1: Print n; "SOUND 0, 1"
n = 3: Sound 19, 1: Print n; "SOUND 19, 1"
n = 4: Sound 20, 0: Print n; "SOUND 20, 0"
n = 5: Sound 32767, 0: Print n; "SOUND 32767, 0"
n = 6: Sound 32768, 1: Print n; "SOUND 32768, 1"
n = 7: Sound -1, 1: Print n; "SOUND -1, 1"
n = 8: Sound 440, -1: Print n; "SOUND 440, -1"
n = 9: Sound 440, 65536: Print n; "SOUND 440, 65536"
n = 10: Sound 440, 0, , , , , 3: Print n; "voice 3"
n = 11: Sound 440, 1, , , , , 4: Print n; "voice 4"
n = 12: Sound 440, 1, , , , , -1: Print n; "voice -1"
n = 13: Sound 10, 1, 2: Print n; "a frequency of 10 and a volume of 2: one error"
Print "checked with a sound device"
n = 20: Sound 440, 0, 1: Print n; "volume 1"
n = 21: Sound 440, 0, 0: Print n; "volume 0"
n = 22: Sound 440, 1, 1.5: Print n; "volume 1.5"
n = 23: Sound 440, 1, -.5: Print n; "volume -.5"
n = 24: Sound 440, 0, , 1: Print n; "pan 1"
n = 25: Sound 440, 0, , -1: Print n; "pan -1"
n = 26: Sound 440, 1, , 1.1: Print n; "pan 1.1"
n = 27: Sound 440, 1, , -1.1: Print n; "pan -1.1"
n = 28: Sound 440, 0, , , 1: Print n; "waveform 1"
n = 29: Sound 440, 0, , , 10: Print n; "waveform 10"
n = 30: Sound 440, 1, , , 0: Print n; "waveform 0"
n = 31: Sound 440, 1, , , 11: Print n; "waveform 11"
n = 32: Sound 440, 1, , , -1: Print n; "waveform -1"
n = 33: Sound 440, 0, , , 9, 0: Print n; "waveform 9 with the parameter 0"
n = 34: Sound 440, 0, , , 9, 1: Print n; "waveform 9 with the parameter 1"
n = 35: Sound 440, 1, , , 9, 1.1: Print n; "waveform 9 with the parameter 1.1"
n = 36: Sound 440, 1, , , 9, -.1: Print n; "waveform 9 with the parameter -.1"
n = 37: Sound 440, 0, , , , 2: Print n; "the parameter 2 alone (the waveform is 9 from before)"
n = 38: Sound 440, 0, , , 4, 2: Print n; "waveform 4 with the parameter 2"
n = 39: Sound 440, 1, 2, , , , 3: Print n; "a volume of 2 in voice 3"
n = 40: Play "H": Print n; "PLAY H"
n = 41: Play "V0 O6 C": Print n; "PLAY O6 C"
n = 42: Play "V0 O7 C": Print n; "PLAY O7 C"
n = 43: Play "V0 T31 C": Print n; "PLAY T31 C"
n = 44: Play "V0 T256 C": Print n; "PLAY T256 C"
n = 45: Play "L0 C": Print n; "PLAY L0 C"
n = 46: Play "L65 C": Print n; "PLAY L65 C"
n = 47: Play "V0 L64 C": Print n; "PLAY L64 C"
n = 48: Play "N85": Print n; "PLAY N85"
n = 49: Play "V0 N84": Print n; "PLAY N84"
n = 50: Play "V101": Print n; "PLAY V101"
n = 51: Play "V100 V0": Print n; "PLAY V100 V0"
n = 52: Play "V0 C D E H C": Print n; "PLAY C D E H C"
n = 53: Play t: Print n; "PLAY of an empty variable"
n = 54: Sound Wait: Print n; "SOUND WAIT a second time"
n = 55: Print n; "nothing to play while held back:"; Play(0); Play(1); Play(2); Play(3)
n = 56: Play "V0 C", "H": Print n; "PLAY with two strings, the second H"
n = 57: Sound Resume: Print n; "SOUND RESUME"
n = 58: Sound Resume: Print n; "SOUND RESUME a second time"
Print "end"
System
handler:
Print n; "error"; Err
Resume Next
