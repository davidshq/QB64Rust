' The mouse in a graphics mode (m3-images-fonts-mouse, task 4.3; mouse steps of the keys file, tools/screen/README.md):
' the queue read event by event with _MOUSEINPUT, and for each event _MOUSEX, _MOUSEY, the three buttons,
' _MOUSEWHEEL and, after the first move, the movement; a dot where each event is. The steps: a click (the spec's
' scenario: four events at 40, 30), a move, the right and the middle button, the wheel down and up, a last move.
' The movement of the first move counts from the real cursor, so it is not printed (design D4).
Dim n As Integer, k As String
Screen 12
While _MouseInput: Wend
GoSub announce
Do: k = InKey$: Loop While k = ""
Print " n   x   y b1 b2 b3 wheel  dx  dy"
Do While _MouseInput
    n = n + 1
    PSet (_MouseX, _MouseY), 10 + n Mod 6
    Print Using "### ### ###"; n; _MouseX; _MouseY;
    Print Using " ## ## ## #####"; _MouseButton(1); _MouseButton(2); _MouseButton(3); _MouseWheel;
    If n > 2 Then Print Using " ### ###"; _MouseMovementX; _MouseMovementY Else Print
Loop
Print "events:"; n; "key: "; k
Print "after:"; _MouseInput; _MouseX; _MouseY; _MouseButton(2); _MouseWheel; _MouseWheel(1)
Print "hidden:"; _MouseHidden; "disabled:"; _MouseDisabled
System
' It moves the machine's cursor, so no test runs it (design D3); its call is compared by tests\callsite.
_MouseMove 10, 20
announce:
Def Seg = 0: Poke 1264, (Peek(1264) + 1) And 255: Def Seg
Return
