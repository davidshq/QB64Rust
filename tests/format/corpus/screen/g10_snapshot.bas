' Cut from Civil War Strategy (tools/screen/cut_game.py): graphics GET and PUT through a SUB on a shared array: the game's snapshot takes the screen under a place and puts it back, here over a drawing that was changed in between, and at a second place.
DefInt A-Z
Dim Shared image(300)
Screen 12
Line (80, 80)-(120, 120), 9, BF
Circle (100, 100), 8, 14
Call snapshot(100, 100, 0)
Line (60, 60)-(140, 140), 4, BF
Call snapshot(100, 100, 1)
Call snapshot(300, 200, 1)
System

Sub snapshot (x, y, flag)
    If flag = 0 Then Get (x - 10, y - 10)-(x + 10, y + 10), image()
    If flag = 1 Then Put (x - 10, y - 10), image(), PSet
End Sub
