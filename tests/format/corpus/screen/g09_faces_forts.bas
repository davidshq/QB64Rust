' Cut from Civil War Strategy (tools/screen/cut_game.py): BLOAD with a file name in a string, eight pictures one after the other into the same two arrays: the faces SUB battle loads and the forts SUB menu loads, each with the game's own three lines, put in two rows.
DefInt A-Z
Dim Shared graft(1 To 1564)
Dim Shared graphic(1 To 1564)
Screen 12
For k = 1 To 5
    If k < 4 Then t$ = "fort" + LTrim$(Str$(k - 1)) + ".vga"
    If k < 4 Then
        Def Seg = VarSeg(graft(1))
        BLoad t$, VarPtr(graft(1))
        Def Seg
        Put (100 * k - 80, 150), graft(), PSet
    End If
    t$ = "face" + LTrim$(Str$(k)) + ".vga"
    Def Seg = VarSeg(graphic(1))
    BLoad t$, VarPtr(graphic(1))
    Def Seg
    Put (100 * k - 80, 20), graphic(), PSet
Next k
System
