' Screen program (m4-qb64pe-builtins, task 2.1): _KEYCLEAR of each buffer. For each of the letters a to d the
' program waits until _KEYHIT has seen it go down, pauses so that it is up too, empties one buffer and prints
' what is left: _KEYCLEAR 1 the INKEY$ buffer, 2 the _KEYHIT buffer, CALL _KEYCLEAR(3) the INP(&H60) buffer, and
' _KEYCLEAR with no argument all three. Then 0 and 4: error 5.
On Error GoTo handler
waitfor 97
_KeyClear 1
Print "1:"; Len(InKey$); _KeyHit
waitfor 98
_KeyClear 2
Print "2:"; _KeyHit; InKey$
waitfor 99
Call _KeyClear(3)
Print "3:"; _KeyHit; InKey$
waitfor 100
_KeyClear
Print "all:"; _KeyHit; Len(InKey$)
_KeyClear 0
_KeyClear 4
Print "end"
System

handler:
Print "error"; Err
Resume Next

Sub waitfor (code&)
    Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
    Do: _Limit 500: Loop Until _KeyHit = code&
    _Delay .3
End Sub
