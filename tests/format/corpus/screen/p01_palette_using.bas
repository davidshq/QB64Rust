' PALETTE USING from a LONG array in SCREEN 13 (libqb takes only LONG there): every attribute gets the levels the
' array holds, from its element 2, and lines are drawn in colours it changed.
Screen 13
Dim b(257) As Long
For i = 0 To 255
    b(i + 2) = 65536 * (i Mod 64) + 256 * ((i \ 4) Mod 64) + (63 - i Mod 64)
Next
Palette Using b(2)
For c = 0 To 15
    Line (c * 20, 0)-(c * 20 + 19, 90), c * 16 + 3, BF
Next
System
