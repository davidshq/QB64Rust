' TEST: cpp
$CONSOLE:ONLY
' LEN, ASC, VAL, HEX$, OCT$, _BIN$ as emitted (m2-core-builtins task 4.2): a string's length field, a place's size
' as a constant, qbs_asc with one and two arguments, the qbs_val parse by the result type, the radix entries with
' the width of a negative value (16 for a 64-bit variable, 0 for a 64-bit expression) or their _float variants
DIM s AS STRING, i AS INTEGER, l AS LONG, q AS _INTEGER64, d AS DOUBLE
PRINT LEN(s); LEN(l); ASC(s); ASC(s, 2)
PRINT VAL(s); VAL(s, SINGLE); VAL(s, DOUBLE); VAL(s, LONG)
PRINT HEX$(i); HEX$(l); HEX$(q); HEX$(q * 1); HEX$(d); OCT$(q); OCT$(i * 1); _BIN$(l); _BIN$(d)
