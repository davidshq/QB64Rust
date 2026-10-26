' Screen program (m4-qb64pe-builtins, task 2.1): _KEYHIT with nothing typed, then the codes of a letter, a capital,
' the up arrow, F1, Enter and the left Shift key going down and up, read until twelve have come. The INKEY$ buffer
' is emptied as the codes come, so no key of the keys file waits on it.
Print _KeyHit
Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
n = 0
Do
    k& = _KeyHit
    If k& Then Print k&;: n = n + 1
    junk$ = InKey$
    _Limit 500
Loop Until n = 12
Print
Print "done"; _KeyHit
System
