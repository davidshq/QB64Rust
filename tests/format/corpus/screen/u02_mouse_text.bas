' The mouse in a text mode (m3-images-fonts-mouse, task 4.3): _MOUSEX and _MOUSEY are the column and row of the
' window pixel of each event, kept inside the screen; the same pixels as in u01_mouse_queue, the corners of the
' window, and a click; an X where each event is. _MOUSEHIDDEN and _MOUSEDISABLED of a window whose cursor
' nothing hid or disabled.
Dim n As Integer, k As String
Screen 0: Width 80, 25
Color 7, 1: Cls
While _MouseInput: Wend
GoSub announce
Do: k = InKey$: Loop While k = ""
Locate 1, 30: Print " n  x  y b1"
Do While _MouseInput
    n = n + 1
    Locate _MouseY, _MouseX: Color 14: Print "X";: Color 7
    Locate 1 + n, 30: Print Using "## ## ## ##"; n; _MouseX; _MouseY; _MouseButton(1)
Loop
Locate 20, 30: Print "events:"; n; "key: "; k
Locate 21, 30: Print "hidden:"; _MouseHidden; "disabled:"; _MouseDisabled
System
announce:
Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
Return
