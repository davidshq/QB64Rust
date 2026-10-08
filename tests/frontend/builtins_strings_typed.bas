' TEST: typed
$CONSOLE:ONLY
' The plain string built-ins (m2-core-builtins task 4.1): LONG slots converted as a store into a LONG (a float
' rounded half to even to _INTEGER64, then truncated), optional slots absent or present, STR$ and _TOSTR$ with an
' any-numeric slot cast to the argument's believed type, STRING$ with a code and with a string
DIM s AS STRING, l AS LONG, i AS INTEGER, q AS _INTEGER64, d AS DOUBLE
PRINT LEFT$(s, 2); RIGHT$(s, l); MID$(s, 2); MID$(s, 2, 3.5); LEFT$(s, q)
PRINT SPACE$(i); STRING$(3, 65); STRING$(l, "xyz"); STRING$(2.5, 65.5)
PRINT LTRIM$(s); RTRIM$(s); _TRIM$(s); UCASE$(s); LCASE$(s + "A")
PRINT STR$(i); STR$(l); STR$(q); STR$(d); STR$(2.5); STR$(i + 1)
PRINT _TOSTR$(d); _TOSTR$(d, 3); _TOSTR$(1 / 3, l)
