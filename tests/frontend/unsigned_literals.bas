' TEST: typed
$CONSOLE:ONLY
' Suffixed literals of the new types (m2-numeric-types design D7, task 5.1): held as C++ types their text, believed
' the suffix's type. `300~%%` is held `int32` and believed `_UNSIGNED _BYTE` (`PRINT` 44, `+ 0` 300); `-1~&&` and
' `-1~`40` get `ull` (wider than 32 bits, unsigned: held 2^64-1); `9`3` is never narrowed (a `_BIT` is printed through
' `int64`); `-9223372036854775808&&` is `-9223372036854775808ll`, which C++ holds `uint64`, believed `_INTEGER64`.
' Same output as qb64pe.exe. The stores into a DOUBLE show each literal's two types (folding hides them in `PRINT`).
d# = 300~%%: d# = -1~&&: d# = 9`3: d# = -1~`40: d# = -9223372036854775808&&
PRINT 300~%%; 300~%% + 0; -1~&&; -1~&& + 0
PRINT 9`3; 9`3 + 0; 0~`40 > -1; -1~`40 > 0
PRINT -9223372036854775808&&; -9223372036854775808&& + 0; -1~& < 0; 4294967295~& + 1
SYSTEM
