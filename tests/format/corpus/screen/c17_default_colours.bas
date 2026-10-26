' Screen program (m4-qb64pe-builtins, task 2.1): _DEFAULTCOLOR and _BACKGROUNDCOLOR after COLOR in SCREEN 0,
' SCREEN 12 and a 32-bit screen, kept and printed on the last screen, and of the screen's page 0 there.
Color 14, 1
a0 = _DefaultColor: b0 = _BackgroundColor
Screen 12
Color 13, 2
a12 = _DefaultColor: b12 = _BackgroundColor
Screen _NewImage(320, 200, 32)
Color _RGB32(200, 100, 50), _RGB32(10, 20, 30)
a32~& = _DefaultColor: b32~& = _BackgroundColor
Print a0; b0
Print a12; b12
Print Hex$(a32~&); " "; Hex$(b32~&)
Print Hex$(_DefaultColor(0)); " "; Hex$(_BackgroundColor(0))
System
