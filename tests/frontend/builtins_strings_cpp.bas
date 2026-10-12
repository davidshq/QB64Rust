' TEST: cpp
$CONSOLE:ONLY
' The plain string built-ins as emitted (m2-core-builtins task 4.1): libqb entries by the table's callname, the
' passed mask of optional slots, any-numeric arguments cast to their C type, STRING$ of a string's first byte
DIM s AS STRING, l AS LONG, i AS INTEGER, d AS DOUBLE
PRINT LEFT$(s, 2); RIGHT$(s, l); MID$(s, 2); MID$(s, 2, 3.5); SPACE$(i)
PRINT STRING$(3, 65); STRING$(l, "xyz"); LTRIM$(s); RTRIM$(s); _TRIM$(s); UCASE$(s); LCASE$(s)
PRINT STR$(i); STR$(5); STR$(2.5); STR$(i + 1); _TOSTR$(d); _TOSTR$(d, 3)
