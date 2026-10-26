' The steps prompt and state of the keys file (tools/screen/README.md). The program announces each wait for a
' key by counting in byte &H4F0 of conventional memory; each prompt of the keys file waits for one announcement,
' and state writes the screen as it is then. A loop that only looks for a key gets none, and a menu that
' announces after every arrow key gets its keys one at a time.
Dim k As String, i As Integer, eaten As Integer, row As Integer
Screen 13
Line (20, 20)-(120, 80), 4, BF
GoSub announce
Do: k = InKey$: Loop While k = ""
Cls
Print "first: "; k
For i = 1 To 100
    _Delay .002
    If InKey$ <> "" Then eaten = eaten + 1
Next i
Print "eaten:"; eaten
GoSub announce
Do: k = InKey$: Loop While k = ""
Print "second: "; k
row = 1
Do
    For i = 1 To 4
        Color 7: If i = row Then Color 14
        Locate 5 + i, 3: Print "entry"; i
    Next i
    GoSub announce
    Do: k = InKey$: Loop While k = ""
    If k = Chr$(0) + "P" Then row = row + 1
Loop Until k = Chr$(13)
Color 15: Locate 11, 1: Print "chosen:"; row
System

announce:
Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
Return
