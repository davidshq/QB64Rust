$CONSOLE:ONLY
' Edge values of the CONST evaluator (m3-constant-evaluation; verification\v45_x25, x32, x36-x40, x60, x61, x65):
' 2 ROOT 0 (infinite, 0 as the whole value), _COTH(0) (libqb's 2 * EXP(x) - 1), _CEIL, INT and ABS beyond
' _INTEGER64 and 0 to a negative power (qbr's indefinite value), a product beyond _FLOAT (0), a palette colour of
' mode 0 beyond its 16 (0, and _ALPHA 255) and of mode 13 (a colour of the 256). Not here: _RED(2222, 0) (x64),
' where libqb reads past its palette ("not supported yet").
CONST x25 = 2 ROOT 0
CONST x32 = _COTH(0)
CONST x36 = _CEIL(1E+30)
CONST x37 = INT(-1E+20)
CONST x38 = 1F+4000 * 1F+4000
CONST x39 = ABS(-9223372036854775807 - 1)
CONST x40 = 0 ^ -1
CONST x60 = _RED(22, 0)
CONST x61 = _RED(200, 13)
CONST x65 = _ALPHA(2222, 0)
PRINT x25; x32; x36; x37; x38; x39; x40
PRINT x60; x61; x65
SYSTEM
