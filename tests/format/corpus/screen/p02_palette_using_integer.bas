' PALETTE USING from an INTEGER array in SCREEN 7: the 16 attributes get the EGA colours the whole array holds, an
' element of -1 leaves its attribute as it was, and boxes are drawn in each attribute.
Screen 7
Dim a(15) As Integer
For i = 0 To 15
    a(i) = (i * 5) Mod 64
Next
a(4) = -1
Palette Using a()
For c = 0 To 15
    Line (c * 20, 0)-(c * 20 + 19, 90), c, BF
Next
System
