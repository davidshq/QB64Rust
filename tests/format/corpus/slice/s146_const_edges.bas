$Console:Only
' Edge values of the CONST evaluator (m3-constant-evaluation; verification\v45_x36, x37, x39, x60, x61, x65):
' _CEIL, INT and ABS beyond _INTEGER64 (qbr's indefinite value), a palette colour of mode 0 beyond its 16 (0, and
' _ALPHA 255) and of mode 13 (a colour of the 256). Not here: _RED(2222, 0) (x64), where libqb reads past its
' palette ("not supported yet"); 2 ROOT 0 and a product beyond _FLOAT (x25, x38: 0 there, an error here,
' QB64PE-DIVERGENCES.md D-029), _COTH(0) (x32: 1 there, an error here, D-031) and 0 ^ -1 (x40: qbr's indefinite value
' there, a division by zero here, D-028), all in tests\frontend\const_rulings_errors.bas (const-rulings).
Const x36 = _Ceil(1E+30)
Const x37 = Int(-1E+20)
Const x39 = Abs(-9223372036854775807 - 1)
Const x60 = _Red(22, 0)
Const x61 = _Red(200, 13)
Const x65 = _Alpha(2222, 0)
Print x36; x37; x39
Print x60; x61; x65
System
