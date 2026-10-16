' TEST: typed
' The checksum, compression and base64 functions (m3-asserts-debug-zlib design D4): _ADLER32 and _CRC32 an
' _UNSIGNED LONG, the others strings; _DEFLATE$'s level a LONG slot (a float rounded), _INFLATE$'s size an
' _INTEGER64 slot, both optional.
$CONSOLE:ONLY
DIM s AS STRING, f AS STRING * 4, k AS LONG, d AS DOUBLE, q AS _INTEGER64, u AS _UNSIGNED LONG
u = _ADLER32(s)
u = _CRC32(f + "x")
s = _DEFLATE$(s)
s = _DEFLATE$(s, d)
s = _INFLATE$(s)
s = _INFLATE$(f, k)
s = _BASE64ENCODE$(s)
s = _BASE64DECODE$(_BASE64ENCODE$("ab"))
SYSTEM
