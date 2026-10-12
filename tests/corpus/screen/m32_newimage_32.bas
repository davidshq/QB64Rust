' A 32-bit screen: opaque and half-transparent pixels, text.
SCREEN _NEWIMAGE(100, 60, 32)
PRINT "mode 32"
LINE (2, 20)-(30, 30), _RGB32(255, 128, 0), BF
PSET (40, 25), _RGB32(1, 2, 3)
PSET (41, 25), _RGBA32(10, 20, 30, 40)
SYSTEM
