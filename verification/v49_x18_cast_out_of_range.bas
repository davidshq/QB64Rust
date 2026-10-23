$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): _CAST of a float constant out of the integer type's range (undefined in C++).
PRINT _CAST(_BYTE, 1000.7); _CAST(LONG, 1E20); _CAST(_UNSIGNED LONG, -1.5)
SYSTEM
