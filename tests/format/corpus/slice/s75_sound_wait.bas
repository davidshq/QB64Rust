$Console:Only
' Slice (m3-sound, task 3.1; from verification\v29_b_sound_wait, v29_g_voice): which sound statements hold the
' program until their sound has played. A time is never printed: it is compared with a limit far from the expected
' value, on the side the load of the machine cannot reach ("over" for a statement that holds the program, "under"
' for one that returns at once). All at volume 0: nothing is heard.
Dim Shared t As Double
Dim e As Double
Call start: Sound 440, 18.2, 0: e = taken#
Print "in the foreground, a SOUND of 1 s holds the program (over 0.4 s): "; yn$(e > .4)
drain 0
Call start: Sound 440, 1, 0: e = taken#
Print "a SOUND of a clock tick does not (under 1 s): "; yn$(e < 1)
Call start: Play "V0 T120 L1 C": e = taken#
Print "a PLAY of 2 s holds it (over 1.2 s): "; yn$(e > 1.2)
drain 0
Play "MB"
Call start: Play "T120 L1 C": e = taken#
Print "after MB the same PLAY returns at once (under 1 s): "; yn$(e < 1); ", with over 1 s left to play: "; yn$(Play(0) > 1)
Call start: Sound 440, 18.2, 0: e = taken#
Print "and so does a SOUND of 1 s (under 1 s): "; yn$(e < 1); ", with over 2 s left: "; yn$(Play(0) > 2)
Play "MF"
Call start: Sound 440, 1, 0: e = taken#
Print "after MF a SOUND waits for what is queued (over 1.5 s): "; yn$(e > 1.5)
drain 0
Sound Wait
Call start: Sound 440, 36.4, 0: e = taken#
Print "after SOUND WAIT a SOUND of 2 s returns at once (under 1 s): "; yn$(e < 1); ", with nothing left to play: "; yn$(Play(0) = 0)
Call start: Sound Resume: e = taken#
Print "SOUND RESUME returns at once (under 1 s): "; yn$(e < 1); ", with over 1 s left: "; yn$(Play(0) > 1)
Play "MB"
Call start: Sound 440, 18.2, 0, , , , 1: e = taken#
Print "voice 0 in the background, voice 1 holds the program (over 0.4 s): "; yn$(e > .4)
drain 1
Play "", "MB V0"
Call start: Sound 440, 36.4, 0, , , , 1: e = taken#
Print "after MB in its own string it does not (under 1 s): "; yn$(e < 1); ", with over 1 s left: "; yn$(Play(1) > 1)
Call start: Play "", , "V0 T120 L1 C": e = taken#
Print "a PLAY with several strings never holds the program (under 1 s): "; yn$(e < 1); ", with over 1 s left in voice 2: "; yn$(Play(2) > 1)
Sound Wait
Sound 440, 36.4, 0, , , , 3
Print "held back by SOUND WAIT, voice 3 has nothing to play: "; yn$(Play(3) = 0)
Play "", ""
Print "a PLAY with several strings ends the SOUND WAIT (over 1 s left): "; yn$(Play(3) > 1)
System

Sub start
    t = Timer(.001)
End Sub

' The seconds since start; a run across midnight is no reason for a "no".
Function taken#
    Dim e As Double
    e = Timer(.001) - t
    If e < 0 Then e = e + 86400
    taken# = e
End Function

' Waits until a voice has played all it has.
Sub drain (voice As Long)
    Do While Play(voice) > 0
        _Delay .05
    Loop
End Sub

Function yn$ (b As Long)
    If b Then yn$ = "yes" Else yn$ = "no"
End Function
