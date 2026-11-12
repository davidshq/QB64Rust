$CONSOLE:ONLY
PRINT "Adler-32 of abc:"; _ADLER32("abc")
PRINT "Of Wikipedia: "; HEX$(_ADLER32("Wikipedia"))
PRINT "Of nothing:"; _ADLER32("")
END
