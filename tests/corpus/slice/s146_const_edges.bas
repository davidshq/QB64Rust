$CONSOLE:ONLY
' Edge values of the CONST evaluator (m3-constant-evaluation; verification\v45_x36, x37, x39, x60, x61, x65):
' _CEIL, INT and ABS beyond _INTEGER64 (qbr's indefinite value), a palette colour of mode 0 beyond its 16 (0, and
' _ALPHA 255) and of mode 13 (a colour of the 256). Not here: _RED(2222, 0) (x64), where libqb reads past its
' palette ("not supported yet"); 2 ROOT 0 and a product beyond _FLOAT (x25, x38: 0 there, an error here,
' DIVERGENCES.md D-029), _COTH(0) (x32: 1 there, an error here, D-031) and 0 ^ -1 (x40: qbr's indefinite value
' there, a division by zero here, D-028), all in tests\frontend\const_rulings_errors.bas (const-rulings).
CONST x36 = _CEIL(1E+30)
CONST x37 = INT(-1E+20)
CONST x39 = ABS(-9223372036854775807 - 1)
CONST x60 = _RED(22, 0)
CONST x61 = _RED(200, 13)
CONST x65 = _ALPHA(2222, 0)
PRINT x36; x37; x39
PRINT x60; x61; x65
SYSTEM
