$CONSOLE:ONLY
PRINT "CRC-32 of abc:"; _CRC32("abc")
PRINT "Of the fox: "; HEX$(_CRC32("The quick brown fox jumps over the lazy dog"))
PRINT "Of nothing:"; _CRC32("")
END
