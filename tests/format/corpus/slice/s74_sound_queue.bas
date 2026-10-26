$Console:Only
' Slice (m3-sound, task 3.1; from verification\v29_f_queue_slice): what each sound statement adds to PLAY(n), the
' seconds left to play of voice n, in the background. PLAY(n) is read directly before and directly after each
' statement and the difference printed in tenths of a second; every length is a whole number of tenths, so a
' reading may come 50 ms late before a number changes. All at volume 0: nothing is heard. BEEP rings the system's
' bell: it is compiled here and never reached.
Dim Shared b As Double, a As Double
Dim t As String, fs As String * 9, i As Integer, l As Long, s As Single, d As Double, q As _Integer64
Print "at start:"; Play(0); Play(1); Play
If Play(0) > 99 Then Beep: Call Beep
Play "MB V0"
Print "SOUND"
b = Play(0): Sound 440, 18.2, 0: a = Play(0): show "SOUND 440, 18.2, 0"
b = Play(0): Sound 880, 9.1, 0: a = Play(0): show "SOUND 880, 9.1, 0"
b = Play(0): Sound 440, 27.3, 0: a = Play(0): show "SOUND 440, 27.3, 0"
b = Play(0): Sound 0, 18.2, 0: a = Play(0): show "SOUND 0, 18.2, 0"
b = Play(0): Sound 0, 18.2,: a = Play(0): show "SOUND 0, 18.2, with a trailing comma"
b = Play(0): Sound 440, 0, 0: a = Play(0): show "SOUND 440, 0, 0"
i = 440: l = 91: s = 9.1: d = 18.2: q = 0
b = Play(0): Sound i, l, q: a = Play(0): show "an INTEGER, a LONG of 91 and an _INTEGER64"
b = Play(0): Sound l * 5, s, q: a = Play(0): show "a LONG, a SINGLE of 9.1"
b = Play(0): Sound d * 100, d, q, q, i - 436, s - 9.1, q: a = Play(0): show "all seven, a DOUBLE of 18.2"
b = Play(0): Call Sound(440, 18.2, 0): a = Play(0): show "CALL SOUND(440, 18.2, 0)"
b = Play(0): Sound 440, 18.2, 0, -1, 3, .5, 0: a = Play(0): show "pan -1, waveform 3"
Print "PLAY"
b = Play(0): Play "T120 L4 C": a = Play(0): show "T120 L4 C"
b = Play(0): Play "C": a = Play(0): show "C (the tempo and the length are kept)"
b = Play(0): Play "T120 L4 C D E": a = Play(0): show "T120 L4 C D E"
b = Play(0): Play "T60 L1 C": a = Play(0): show "T60 L1 C"
b = Play(0): Play "T60 L4 C.": a = Play(0): show "T60 L4 C."
b = Play(0): Play "T120 L4 C2": a = Play(0): show "T120 L4 C2"
b = Play(0): Play "T120 P4": a = Play(0): show "T120 P4"
b = Play(0): Play "T120 L4 N0": a = Play(0): show "T120 L4 N0"
b = Play(0): Play "T120 L4 N37": a = Play(0): show "T120 L4 N37"
b = Play(0): Play "MS T120 L4 C": a = Play(0): show "MS T120 L4 C"
b = Play(0): Play "ML T120 L4 C": a = Play(0): show "ML T120 L4 C"
b = Play(0): Play "MN T120 L4 C": a = Play(0): show "MN T120 L4 C"
b = Play(0): Play "t120 l4 o3 c# d- e+ >c <c": a = Play(0): show "t120 l4 o3 c# d- e+ >c <c"
b = Play(0): Play "": a = Play(0): show "the empty string"
b = Play(0): Play "MST150o1e8o0b8o1e8e8": a = Play(0): show "MST150o1e8o0b8o1e8e8"
t = "T120 L4 C"
b = Play(0): Play t: a = Play(0): show "a variable"
b = Play(0): Play t + " " + Mid$(t, 9): a = Play(0): show "an expression"
b = Play(0): Play t,: a = Play(0): show "a trailing comma"
b = Play(0): Call Play(t): a = Play(0): show "CALL PLAY(t)"
fs = t
b = Play(0): Play fs: a = Play(0): show "a fixed-length string"
Print "voices"
Play "", "MB V0", "MB V0", "MB V0"
b = Play(1): Sound 440, 18.2, 0, , , , 1: a = Play(1): show "voice 1: SOUND 440, 18.2, 0, , , , 1"
b = Play(1): Play "T120 L4 C", "T120 L2 C": a = Play(1): show "voice 1: the second string, T120 L2 C"
b = Play(2): Play "", , "T120 L2 C": a = Play(2): show "voice 2: the third string"
b = Play(3): Play "", , , fs: a = Play(3): show "voice 3: the fourth string, a fixed-length string"
b = Play(3): Play "", "T120 L4 C", "T120 L4 C", "T120 L4 C": a = Play(3): show "voice 3: of four strings"
Print "voice 0 has over 5 s left:"; Play(0) > 5; Play > 5; " as have the voices 4 and -1:"; Play(4) > 5; Play(-1) > 5
i = 2: s = 1
Print "voices 1 and 2 by a variable:"; Play(s) > 1; Play(i) > .5
Print "SOUND WAIT"
Sound Wait
b = Play(1): Sound 440, 18.2, 0, , , , 1: a = Play(1): show "held back: SOUND 440, 18.2, 0, , , , 1"
b = Play(1): Sound Resume: a = Play(1): show "SOUND RESUME"
b = Play(1): Sound 440, 18.2, 0, , , , 1 Wait: a = Play(1): show "SOUND 440, 18.2, 0, , , , 1 WAIT"
b = Play(1): Sound 880, 9.1, 0, , , , 1 Resume: a = Play(1): show "SOUND 880, 9.1, 0, , , , 1 RESUME"
System

Sub show (what As String)
    Print what; ":"; CLng((a - b) * 10)
End Sub
