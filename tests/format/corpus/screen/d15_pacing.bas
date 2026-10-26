' _LIMIT, _DELAY, SLEEP and _DISPLAY: nothing in the state depends on how long anything took.
Dim i As Integer
Screen 7
For i = 1 To 10
    PSet (i * 10, 50), i
    _Limit 60
Next i
_Delay .01
_Display
Line (10, 60)-(100, 70), 4, BF
_Display
' A key ends SLEEP only when it comes while SLEEP waits, and the keys file gives the next key only when the
' key buffer is empty: so the buffer is emptied first. The time is the longest SLEEP can take, not what it takes.
Do: Loop Until InKey$ = ""
Sleep 2
Print "after SLEEP"
System
