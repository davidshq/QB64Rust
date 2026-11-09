$CONSOLE:ONLY
' Verification (study 44, section 2 item 1): a FUNCTION and a SUB of the same name, the FUNCTION first, as in
' InForm's extensions\HMap64.bi. The SUB calls the FUNCTION and drops its result.
PRINT f(2)
f 3
CALL f(4)
SYSTEM

FUNCTION f%% (n AS INTEGER)
    PRINT "function"; n;
    f = n * 2
END FUNCTION

SUB f (n AS INTEGER)
    DIM ignored AS _BYTE: ignored = f(n)
    PRINT "sub"; n
END SUB
