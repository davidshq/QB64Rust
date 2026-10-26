' The steps rule and play until of the keys file (tools/screen/README.md). Before each wait for a key the program
' writes the place of the wait to byte &H4F1 and a letter to &H4F2, then announces the wait in &H4F0. Month k
' brings k battles, each a wait at place 7; the second battle of a month asks at a menu (place 14, letter R) and
' then for a confirmation (place 5); the third month also passes a menu with the letter U; each month ends at the
' main menu (place 14, letter M). The keys file does not count the waits: it answers them by their places.
Dim k As String, month As Integer, b As Integer
Screen 12
For month = 1 To 3
    For b = 1 To month
        Print "month"; month; "battle"; b;
        k = waitkey$(7, 0): Print " got "; shown$(k);
        If b = 2 Then
            k = waitkey$(14, Asc("R")): Print ", retreat menu got "; shown$(k);
            k = waitkey$(5, 0): Print ", confirm got "; shown$(k);
        End If
        Print
    Next b
    If month = 3 Then k = waitkey$(14, Asc("U")): Print "utility menu got "; shown$(k)
    k = waitkey$(14, Asc("M")): Print "main menu got "; shown$(k)
Next month
System

Function waitkey$ (place As Integer, letter As Integer)
    Dim k As String
    Def Seg = 0: Poke 1265, place: Poke 1266, letter: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
    Do: k = InKey$: Loop While k = ""
    waitkey$ = k
End Function

Function shown$ (k As String)
    If Asc(k) < 33 Then shown$ = "CHR$(" + LTrim$(Str$(Asc(k))) + ")" Else shown$ = k
End Function
