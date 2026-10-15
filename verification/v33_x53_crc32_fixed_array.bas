$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): does the old compiler accept this? (53_crc32_fixed_array)
DIM a(3) AS STRING * 4
PRINT _CRC32(a(1))
SYSTEM
