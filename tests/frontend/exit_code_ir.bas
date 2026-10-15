' TEST: ir
' END and SYSTEM with an exit code (m3-asserts-debug-zlib design D5): a float rounded to LONG through
' qbr_float_to_long, an integer as it is, an expression, in a SUB, in a one-line IF.
$CONSOLE:ONLY
DIM k AS LONG, d AS DOUBLE, q AS _INTEGER64, o AS _OFFSET
IF k THEN SYSTEM 2.5 ELSE END d
SYSTEM k
END q + 1
SYSTEM o
s
SUB s
    SYSTEM 3
END SUB
