' TEST: check-fail
$CONSOLE:ONLY
' Wider than 64 bits: an overflow error, never a value wrapped into range (2^128 - 1 used to become -1).
' `-9223372036854775808&&` is held as C++ holds `-9223372036854775808ll`, an `_UNSIGNED _INTEGER64`, and believed
' `_INTEGER64` (design D7 of m2-numeric-types): it compiles (`unsigned_literals`, typed, shows its two types).
PRINT 340282366920938463463374607431768211455&&
PRINT -340282366920938463463374607431768211455&&
PRINT 340282366920938463463374607431768211455
PRINT 9223372036854775807&&; -9223372036854775808&&
END
