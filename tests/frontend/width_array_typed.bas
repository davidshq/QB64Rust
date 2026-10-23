' TEST: typed
$CONSOLE:ONLY
' An array named WIDTH (m4-names-implicit-arrays task 3.2, design D8): `WIDTH (n) = v` stores into an implicit
' array `WIDTH`, with or without a blank before the parenthesis, and `WIDTH(n)` in an expression reads it (WIDTH is
' no function); the statement keeps its forms (measured, verification\v27_x51, v55_x66, x67).
WIDTH (3) = 5
WIDTH(1) = WIDTH(3) + 2
PRINT WIDTH(1); UBOUND(WIDTH)
WIDTH 80
SYSTEM
