' TEST: typed
$CONSOLE:ONLY
' Suffixed literals and constants out of their type's range (m2-numeric-types design D7, task 4.2): held as C++
' types their digits, believed the suffix's type. `PRINT` converts to the believed type (`32768%` prints -32768,
' `small%` -25536), an operation and a store use the held value (`32768% + 0` is 32768). `-2147483648&` is held in
' 64 bits, as C++ reads `-(2147483648)`; a suffixed literal inside a CONST is its held value (`r` is 40000).
CONST small% = 40000, c& = 3000000000, r = 40000%, plain = 40000
PRINT small%; small% + 0; c&; c& + 0; r; plain
PRINT 32768%; 32768% + 0; -2147483648&; (-2147483648&) * (-32768%); 2147483647&& + 1; 2147483647& + 1
PRINT &HFFFF%; &HFFFF& + 0; HEX$(40000%); ABS(40000%)
x& = 40000%
y% = 40000
SYSTEM
