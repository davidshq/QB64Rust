' Screen program (m4-qb64pe-builtins, task 2.1): _KEYDOWN of a held key. The left Shift key: the program waits
' until it is down, prints it, waits until it is up and prints it again, then the _KEYHIT codes it left. The
' space bar: down (the left Shift key not), up, and what INKEY$ got.
announce
Do: _Limit 500: Loop Until _KeyDown(100304)
Print _KeyDown(100304);
Do: _Limit 500: Loop While _KeyDown(100304)
Print _KeyDown(100304)
Print _KeyHit; _KeyHit; _KeyHit
announce
Do: _Limit 500: Loop Until _KeyDown(32)
Print _KeyDown(32); _KeyDown(100304);
Do: _Limit 500: Loop While _KeyDown(32)
Print _KeyDown(32)
Print "["; InKey$; "]"
System

Sub announce
    Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
End Sub
