' TEST: typed
$CONSOLE:ONLY
' A FUNCTION of a `_BIT * n` type is called as any other (verification\v21_a_functions, compared by
' verification-compare); only a one-bit FUNCTION's call is "Name already in use" (v21_x16, x17, x47, in
' numeric_decl_errors.bas).
PRINT fb3`3(5); fb3`3(3.5)
PRINT fub3~`3(13); fub3~`3(-1)
PRINT HEX$(fb3`3(-2))
SYSTEM

FUNCTION fb3`3 (x)
fb3`3 = x
END FUNCTION

FUNCTION fub3~`3 (x)
fub3~`3 = x
END FUNCTION
