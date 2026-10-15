$CONSOLE:ONLY
' Slice (m3-asserts-debug-zlib task 3.1): the checksum, compression and base64 functions (verification\v33_b01,
' b02): checksums of strings of each kind, every compression level, a size given to _INFLATE$ (fewer bytes cut;
' more, and the bytes past the data are not set, so only their count is printed), bytes that are no zlib stream
' (zero bytes, no error), base64 of each length and of bad input.
ON ERROR GOTO handler
DIM s AS STRING, c AS STRING, k AS LONG, d AS DOUBLE, i64 AS _INTEGER64, f AS STRING * 4, u AS _UNSIGNED LONG
PRINT _CRC32("abc"); _ADLER32("abc")
PRINT _CRC32(""); _ADLER32("")
PRINT HEX$(_CRC32("The quick brown fox jumps over the lazy dog")); " "; HEX$(_ADLER32("Wikipedia"))
f = "abc"
PRINT _CRC32(f); _ADLER32(f)
k = _CRC32("abc"): PRINT k
u = _ADLER32(s + "x"): PRINT u
i64 = _CRC32("abc") * 2: PRINT i64
PRINT LEN(_DEFLATE$("this is a test", 9)); LEN(_DEFLATE$("this is a test"))
s = STRING$(1000, "a")
FOR k = -2 TO 12
    c = _DEFLATE$(s, k)
    PRINT k; LEN(c); _INFLATE$(c) = s
NEXT
c = _DEFLATE$(s)
PRINT LEN(c); LEN(_INFLATE$(c)); LEN(_INFLATE$(c, 1000)); LEN(_INFLATE$(c, 10)); LEN(_INFLATE$(c, 5000))
PRINT LEN(_INFLATE$(c, 0)); LEN(_INFLATE$(c, -1)); LEFT$(_INFLATE$(c, 1005), 1000) = s
PRINT HEX$(ASC(c, 1)); HEX$(ASC(c, 2)); LEN(_DEFLATE$(""))
PRINT LEN(_INFLATE$(_DEFLATE$("")))
PRINT _INFLATE$(_DEFLATE$("abc")) = "abc"; _BASE64DECODE$(_BASE64ENCODE$("abc"))
d = 3.7: PRINT LEN(_DEFLATE$(s, d)); LEN(_INFLATE$(c, d))
c = _DEFLATE$("hello world")
PRINT "["; _INFLATE$(c, 3); "]"; LEN(_INFLATE$(LEFT$(c, 4), 5)); LEN(_INFLATE$("not deflated", 3))
s = _INFLATE$("not deflated")
PRINT LEN(s); s = STRING$(LEN(s), 0)
PRINT "["; _BASE64ENCODE$(""); "]["; _BASE64ENCODE$("a"); "]["; _BASE64ENCODE$("ab"); "]["; _BASE64ENCODE$("abc"); "]"
PRINT _BASE64ENCODE$(CHR$(0) + CHR$(255) + CHR$(254) + CHR$(62) + CHR$(63)); " "; _BASE64ENCODE$(f)
PRINT "["; _BASE64DECODE$(""); "]["; _BASE64DECODE$("YQ=="); "]["; _BASE64DECODE$("YQ"); "]["; _BASE64DECODE$("YWJj"); "]"
PRINT "["; _BASE64DECODE$("!!!!"); "]"; LEN(_BASE64DECODE$("Y Q = ="))
PRINT LEN(_BASE64DECODE$("YWJ")); LEN(_BASE64DECODE$("YWJjZ")); LEN(_BASE64DECODE$("YW=j"))
PRINT "done"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
SUB never
    PRINT _INFLATE$("x")
END SUB
