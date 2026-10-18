$CONSOLE:ONLY
' Verification (m3-constant-evaluation, before its proposal): the constants of the auto-included file that were not
' computed yet: _E and the limits of the types
PRINT _TOSTR$(_E); _E
PRINT _INTEGER64_MIN; _INTEGER64_MIN + 0
PRINT _UINTEGER64_MAX; _UINTEGER64_MAX + 0
PRINT _OFFSET_MIN; _UOFFSET_MAX
PRINT _TOSTR$(_DOUBLE_MAX); " "; _TOSTR$(_DOUBLE_MIN)
PRINT _TOSTR$(_FLOAT_MIN_FRAC); " "; _TOSTR$(_FLOAT_MAX); " "; _TOSTR$(_FLOAT_MIN)
SYSTEM
