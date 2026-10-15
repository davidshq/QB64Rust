$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): values of _ADLER32, _CRC32, _DEFLATE$, _INFLATE$, _BASE64ENCODE$, _BASE64DECODE$.
DIM s AS STRING, c AS STRING, k AS LONG, i AS INTEGER, d AS DOUBLE, i64 AS _INTEGER64, f AS STRING * 4
PRINT _CRC32("abc"); _ADLER32("abc")
PRINT _CRC32(""); _ADLER32("")
PRINT HEX$(_CRC32("The quick brown fox jumps over the lazy dog")); HEX$(_ADLER32("Wikipedia"))
f = "abc"
PRINT _CRC32(f); _ADLER32(f)
k = _CRC32("abc"): PRINT k
i64 = _CRC32("abc") * 2: PRINT i64
PRINT LEN(_DEFLATE$("this is a test", 9)); LEN(_DEFLATE$("this is a test"))
s = STRING$(1000, "a")
FOR k = -2 TO 12
    c = _DEFLATE$(s, k)
    PRINT k; LEN(c); _INFLATE$(c) = s
NEXT
c = _DEFLATE$(s)
PRINT LEN(c); LEN(_INFLATE$(c)); LEN(_INFLATE$(c, 1000)); LEN(_INFLATE$(c, 10)); LEN(_INFLATE$(c, 5000))
PRINT LEN(_INFLATE$(c, 0)); LEN(_INFLATE$(c, -1))
PRINT HEX$(ASC(c, 1)); HEX$(ASC(c, 2)); LEN(_DEFLATE$(""))
PRINT LEN(_INFLATE$(_DEFLATE$("")))
PRINT _INFLATE$(_DEFLATE$("abc")) = "abc"; _BASE64DECODE$(_BASE64ENCODE$("abc"))
PRINT "["; _BASE64ENCODE$(""); "]["; _BASE64ENCODE$("a"); "]["; _BASE64ENCODE$("ab"); "]["; _BASE64ENCODE$("abc"); "]"
PRINT _BASE64ENCODE$(CHR$(0) + CHR$(255) + CHR$(254) + CHR$(62) + CHR$(63))
PRINT "["; _BASE64DECODE$(""); "]["; _BASE64DECODE$("YQ=="); "]["; _BASE64DECODE$("YQ"); "]["; _BASE64DECODE$("YWJj"); "]"
PRINT "["; _BASE64DECODE$("!!!!"); "]"; LEN(_BASE64DECODE$("!!!!")); LEN(_BASE64DECODE$("Y Q = ="))
PRINT LEN(_BASE64DECODE$("YWJ")); LEN(_BASE64DECODE$("YWJjZ")); LEN(_BASE64DECODE$("YW=j"))
d = 3.7: PRINT LEN(_DEFLATE$(s, d)); LEN(_INFLATE$(c, d))
PRINT "done"
SYSTEM
