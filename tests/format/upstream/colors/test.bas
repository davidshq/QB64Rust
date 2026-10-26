$Console:Only

Print "RGB functions forward"
Print "Grey, max alpha: "; Hex$(_RGB32(127))
Print "Grey, half alpha: "; Hex$(_RGB32(127, 127))
Print "Grey, max alpha: "; Hex$(_RGB32(127, 127, 127))
Print "Grey, half alpha: "; Hex$(_RGB32(127, 127, 127, 127))

c~& = _RGBA32(80, 128, 160, 32)
Print
Print "RGB functions backward _RGBA32(80, 128, 160, 32)"
Print "Red..:"; _Red32(c~&)
Print "Green:"; _Green32(c~&)
Print "Blue.:"; _Blue32(c~&)
Print "Alpha:"; _Alpha32(c~&)

Print
Print "HSB functions forward"
Print "Red, max alpha: "; Hex$(_HSB32(0, 100, 100))
Print "Red, half alpha: "; Hex$(_HSBA32(0, 100, 100, 50))
Print "Midgreen, max alpha: "; Hex$(_HSB32(120, 75, 75))
Print "Midgreen, 3/4 alpha: "; Hex$(_HSBA32(120, 75, 75, 75))

c~& = _HSBA32(90, 75, 65, 55)
Print
Print "HSB functions backward _HSBA32(90, 75, 65, 55)"
Print "Hue.......:"; CInt(_Hue32(c~&))
Print "Saturation:"; CInt(_Saturation32(c~&))
Print "Brightness:"; CInt(_Brightness32(c~&))
Print "Alpha.....:"; CInt(_Alpha32(c~&) / 255 * 100)

System

