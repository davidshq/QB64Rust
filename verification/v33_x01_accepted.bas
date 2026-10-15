$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): does the old compiler accept this? (01_accepted)
$ASSERTS
DIM s AS STRING, k AS LONG, i AS INTEGER, d AS DOUBLE, i64 AS _INTEGER64, f AS STRING * 4, b AS _BYTE
DIM u AS _UNSIGNED LONG, sg AS SINGLE
PRINT "ok"
SYSTEM
_ASSERT k
_ASSERT k, s
_ASSERT d, "m"
_ASSERT i64 = 1, f
_ASSERT (k)
CALL _ASSERT(k)
CALL _ASSERT(k, s)
u = _ADLER32(s)
u = _CRC32(f)
k = _ADLER32(s + "x")
s = _DEFLATE$(s)
s = _DEFLATE$(s, k)
s = _DEFLATE$(s, d)
s = _DEFLATE$(s, i64)
s = _DEFLATE$(f, b)
s = _INFLATE$(s)
s = _INFLATE$(s, k)
s = _INFLATE$(s, d)
s = _INFLATE$(s, i64)
s = _INFLATE$(s, u)
s = _BASE64ENCODE$(s)
s = _BASE64DECODE$(f)
PRINT _CRC32(s); _BASE64ENCODE$(s)
SYSTEM k
SYSTEM d
SYSTEM i64
SYSTEM u
SYSTEM sg
END k
END 1 + 2
SYSTEM (3)
