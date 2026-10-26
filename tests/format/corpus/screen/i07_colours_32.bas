' The colour functions in 32 bits (m3-images-fonts-mouse, task 2.4; verification\v31_a_colours): _RGB and _RGBA
' of the screen and of an image, _RED to _ALPHA with and without a handle, the 32-bit parts, _HSB32 and _HSBA32,
' _HUE32, _SATURATION32 and _BRIGHTNESS32; a box in each colour made and each value printed.
Screen _NewImage(240, 220, 32)
t& = _NewImage(4, 4, 32)
p& = _NewImage(4, 4, 256)
c~& = _RGB(255, 128, 0)
Line (0, 0)-(15, 15), c~&, BF
Line (20, 0)-(35, 15), _RGBA(0, 128, 255, 128), BF
Line (40, 0)-(55, 15), _RGB(10, 200, 30, t&), BF
Line (60, 0)-(75, 15), _HSB32(120, 100, 100), BF
Line (80, 0)-(95, 15), _HSBA32(300, 50, 75, 50), BF
Line (100, 0)-(115, 15), _RGB32(1, 2, 3), BF
Locate 3, 1
Print Hex$(c~&); " "; Hex$(_RGBA(1, 2, 3, 4, t&)); " "; Hex$(_RGB(300, -5, 128))
Print _RGB(255, 0, 0, p&); _RGBA(0, 0, 170, 9, p&)
Print _Red(&HFF102030); _Green(&HFF102030); _Blue(&HFF102030); _Alpha(&H80102030)
Print _Red(&HFF102030, t&); _Green(4, p&); _Alpha(&H7F000000, t&)
Print _Red32(&HFF102030); _Green32(&HFF102030); _Blue32(&HFF102030); _Alpha32(&H80102030)
Print Hex$(_HSB32(0, 100, 100)); " "; Hex$(_HSB32(240, 50, 50))
Print Hex$(_HSBA32(60, 100, 100, 25)); " "; Hex$(_HSB32(400, 120, -10))
Print _Hue32(&HFF00FF00); _Hue32(&HFF0000FF)
Print _Saturation32(&HFF804040); _Brightness32(&HFF804040)
Print _Hue32(&HFF808080); _Saturation32(&HFF808080); _Brightness32(0)
System
