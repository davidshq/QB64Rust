$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): does the old compiler accept this? (48_system_udt)
TYPE t
    a AS LONG
END TYPE
DIM v AS t
SYSTEM v
