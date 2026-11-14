$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x02_base64decode_bare)
' _BASE64DECODE$ with no argument.
k = 0
IF k = 1 THEN
    PRINT _BASE64DECODE$
END IF
PRINT "ok"
SYSTEM
