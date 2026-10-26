' Cut from Civil War Strategy (tools/screen/cut_game.py): BLOAD, PUT and a GET out of the picture just drawn: the game's icon and the capital marker its main module takes from it, then the marker put in a row as SUB showcity puts it on a city.
DefInt A-Z
Dim Shared graphic(1 To 1564), Ncap(60)
Screen 12
Def Seg = VarSeg(graphic(1))
BLoad "cwsicon.vga", VarPtr(graphic(1))
Def Seg

Put (100, 100), graphic(), PSet
Get (101, 101)-(113, 113), Ncap()
For i = 0 To 5: Put (300 + 20 * i, 200 + 5 * i), Ncap(), PSet: Next i
System
