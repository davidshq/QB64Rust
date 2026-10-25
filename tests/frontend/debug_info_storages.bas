' TEST: debug-info
$CONSOLE:ONLY
' The debug symbol file (m5-debug-build-symbols task 2.2, spec compiler/debug-info "Each storage", "A static
' array", "A dynamic array", "A type as BASIC spells it"): a variable of each storage (main, static, local, param,
' result), a static and a dynamic array, a string, a _BIT * 3 and an _UNSIGNED INTEGER.
DIM SHARED total AS LONG
DIM a(1 TO 3, 0 TO 1) AS SINGLE
n = 2
REDIM b(n) AS STRING
DIM s AS STRING
DIM bits AS _BIT * 3
DIM u AS _UNSIGNED INTEGER
CALL show(5)
PRINT f&; a(1, 0); b(0); s; bits; u
SUB show (n AS LONG)
    STATIC calls AS INTEGER
    x = n + 1
    calls = calls + 1
    total = total + x
END SUB
FUNCTION f&
    f& = total
END FUNCTION
