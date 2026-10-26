Dim h As Long
h = 1
_SndPlay h
Do While _SndPlaying(h)
    _Limit 60
Loop
