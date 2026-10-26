' The step digest of the keys file (tools/screen/README.md): a line with a hash of the state that a state step
' would write at that moment, in its place among the states. The screen changes between the two digests.
Dim k As String
Screen 12
Line (10, 10)-(200, 100), 4, BF
Print "first"
GoSub announce
Do: k = InKey$: Loop While k = ""
Circle (300, 200), 50, 14
Print "second: "; k
GoSub announce
Do: k = InKey$: Loop While k = ""
Print "third: "; k
GoSub announce
Do: k = InKey$: Loop While k = ""
System

announce:
Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
Return
