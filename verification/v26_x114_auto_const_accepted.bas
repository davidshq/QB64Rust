$CONSOLE:ONLY
' Verification (step 11, group 5): the constants of the auto-included file with their own suffix and with another, the ones the
' new compiler does not compute yet (their values are recorded here), in a CONST, as a string length.
DIM n AS LONG, d AS DOUBLE
PRINT _TRUE%; _TRUE&&; _TRUE#; _ONE_KB&&; _LONG_MAX; _LONG_MAX%
PRINT _BIT_MIN; _BIT_MIN`; _E; _E#; _E##
PRINT _DOUBLE_MAX; _DOUBLE_MIN; _FLOAT_MAX; _FLOAT_MIN; _FLOAT_MIN_FRAC
PRINT _INTEGER64_MIN; _UINTEGER64_MAX; _OFFSET_MIN; _UOFFSET_MAX
CONST k = _TRUE + 1
CONST m = _ONE_KB&&
CONST j = _LONG_MAX
DIM s AS STRING * _ONE_KB
PRINT k; m; j; LEN(s); LEN(_STR_EMPTY); LEN(_STR_CRLF); ASC(_CHR_CR); LEN(_STR_NAT_EOL)
PRINT "ok"
SYSTEM
