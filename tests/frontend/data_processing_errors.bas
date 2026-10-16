' TEST: check-fail
' The forms of the checksum, compression and base64 functions the old compiler rejects (m3-asserts-debug-zlib
' design D4; verification/v33_x30-x36, x45, x46): a number for a string, a string for the level or the size, a
' wrong count, empty parentheses, the bare name. Each is an error of the program, none "not supported yet"; the
' messages are new.
$CONSOLE:ONLY
PRINT _ADLER32(5)
PRINT _DEFLATE$("a", "b")
PRINT _DEFLATE$("a", 1, 2)
PRINT _INFLATE$(5)
PRINT _DEFLATE$()
PRINT _ADLER32
PRINT _CRC32("a", "b")
PRINT _BASE64ENCODE$("a", 1)
PRINT _INFLATE$("a", "b")
SYSTEM
