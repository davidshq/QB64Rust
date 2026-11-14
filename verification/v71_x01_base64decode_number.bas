$CONSOLE:ONLY
' Verification (language-reference, second review): does the old compiler accept this? (x01_base64decode_number)
' _BASE64DECODE$ with a number.
k = 0
IF k = 1 THEN
    PRINT _BASE64DECODE$(5)
END IF
PRINT "ok"
SYSTEM
