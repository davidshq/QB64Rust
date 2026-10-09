' TEST: cpp
$CONSOLE:ONLY
' The C++ of the decided fixes of m2-numeric-types design D8: `a IMP b IMP c` written as (a IMP b) IMP c (D-005),
' and the division templates that test a divisor of -1 (D-006).
a = 5: b = 3: d = 0
PRINT a IMP b IMP d
l& = -2147483648: m& = -1
PRINT l& \ m&; l& MOD m&
SYSTEM
