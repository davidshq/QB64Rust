$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): does the old compiler accept this? (x22_acceptfiledrop_fn_arg)
' the function _ACCEPTFILEDROP with an argument.
k = 0
IF k = 1 THEN
    x = _ACCEPTFILEDROP(1)
END IF
PRINT "ok"
SYSTEM
