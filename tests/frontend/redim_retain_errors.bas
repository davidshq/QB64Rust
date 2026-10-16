' TEST: check-fail
$CONSOLE:ONLY
' The words after REDIM (m3-redim-retain task 2.2, design D2; spec language/arrays-and-types "REDIM"): one of
' _PRESERVE and _RETAIN, then SHARED. Every other order is "Invalid variable name" in the old compiler
' (verification\v34_x03 to x05, x22), an error at the second word here; REDIM _RETAIN SHARED in a SUB is "DIM/REDIM
' SHARED invalid within a SUB/FUNCTION" (x13). The lines between them are accepted.
REDIM a(3) AS LONG
REDIM _RETAIN SHARED a(5) AS LONG
REDIM SHARED _RETAIN a(5) AS LONG
REDIM SHARED _PRESERVE a(5) AS LONG
REDIM _PRESERVE _RETAIN a(5) AS LONG
REDIM _RETAIN _PRESERVE a(5) AS LONG
REDIM _RETAIN _RETAIN a(5) AS LONG
REDIM _PRESERVE _PRESERVE a(5) AS LONG
REDIM SHARED SHARED a(5) AS LONG
REDIM _RETAIN a(1 TO 6) AS LONG
SYSTEM
SUB s
    REDIM _RETAIN SHARED g(3) AS LONG
END SUB
