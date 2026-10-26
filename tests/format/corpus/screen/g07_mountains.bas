' Cut from Civil War Strategy (tools/screen/cut_game.py): BLOAD of a picture into an array in conventional memory (DEF SEG, VARSEG, VARPTR) and graphics PUT with PSET: the game's mountains, loaded as its main module loads them and drawn as the first lines of SUB usa draw them.
DefInt A-Z
Dim Shared mtn(1 To 1564)
Def Seg = VarSeg(mtn(1))
BLoad "mtn.vga", VarPtr(mtn(1))
Def Seg
Screen 12
Line (1, 16)-(527, 440), 10, B
Paint (200, 200), 2, 10
Color 10
'==========================  Mountains =================================
If _FileExists("mtn.vga") Then
    Put (380, 30), mtn(), PSet
    Put (270, 200), mtn(), PSet
    Put (310, 160), mtn(), PSet
    Put (350, 120), mtn(), PSet
    Put (200, 185), mtn(), PSet
    Put (250, 130), mtn(), PSet
    Put (320, 80), mtn(), PSet
    Put (30, 150), mtn(), PSet
    Line (30, 150)-(70, 190), 2, BF
End If
System
