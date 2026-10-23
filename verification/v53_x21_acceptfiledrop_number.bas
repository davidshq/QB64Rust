$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x21_acceptfiledrop_number)
' _ACCEPTFILEDROP 1.
k = 0
IF k = 1 THEN
    _ACCEPTFILEDROP 1
END IF
PRINT "ok"
SYSTEM
