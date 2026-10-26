' Screen program (m3-events, task 3.3): ON KEY in a window, given its keys by the keys file. F1, the up arrow
' and a user key (KEY 15 with flags 0 and the scancode of A) run their handlers; a key switched off reaches
' INKEY$ instead; the soft-key line of KEY ON and the list of KEY LIST.
Dim Shared got As Integer
Dim k As String
Key 15, Chr$(0) + Chr$(30)
Key 2, "two"
On Key(1) GoSub f1
On Key(11) GoSub up
On Key(15) GoSub user
Key(1) On
Key(11) On
Key(15) On
Key On
Print "waiting for F1"
GoSub announce
Do: _Limit 100: Loop Until got = 1
GoSub announce
Do: _Limit 100: Loop Until got = 2
GoSub announce
Do: _Limit 100: Loop Until got = 3
Key(1) Off
Print "F1 is off"
GoSub announce
Do: k = InKey$: _Limit 100: Loop While k = ""
Print "INKEY$ got"; Len(k); "bytes:"; Asc(k, Len(k))
Key Off
Key List
System

f1:
got = got + 1
Print "F1 handler"
Return

up:
got = got + 1
Print "up handler"
Return

user:
got = got + 1
Print "user key handler"
Return

announce:
Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
Return
