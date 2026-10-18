' TEST: cpp
$CONSOLE:ONLY
' Float constants in the C++ (m3-constant-evaluation task 2.3, design D2): the text the old compiler writes, its
' _TOSTR$ digits as they are (a point and 0 added, E+0 after plain digits), with L for a _FLOAT, which is a float
' result whose text has an exponent. Read from the old compiler's C++ of the same constants (verification
' v45_a_precision): 3.333333333333333333E+29L, 0.3333333333333333333E+0, 3.333333333333333333E-11L,
' 1.414213562373095049E+0; a ## constant and an integer-valued one beside them.
CONST c = 1E+30 / 3
CONST d = 1 / 3
CONST e = 1E-10 / 3
CONST w = 2 ^ 0.5
CONST third## = 1## / 3##
CONST two = 4 / 2
PRINT _TOSTR$(c); _TOSTR$(d); _TOSTR$(e); _TOSTR$(w)
PRINT _TOSTR$(third##); two
SYSTEM
