' A 32-bit screen: opaque and half-transparent pixels, text.
Screen _NewImage(100, 60, 32)
Print "mode 32"
Line (2, 20)-(30, 30), _RGB32(255, 128, 0), BF
PSet (40, 25), _RGB32(1, 2, 3)
PSet (41, 25), _RGBA32(10, 20, 30, 40)
System
