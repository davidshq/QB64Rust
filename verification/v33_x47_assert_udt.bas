$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): does the old compiler accept this? (47_assert_udt)
TYPE t
    a AS LONG
END TYPE
DIM v AS t
_ASSERT v
PRINT "ok"
SYSTEM
