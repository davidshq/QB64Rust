' TEST: typed
$CONSOLE:ONLY
' numeric-semantics: integer arithmetic width and wrap, also in folding
i% = 32767: PRINT i% + 1
x& = 2147483647: PRINT x& + 1
PRINT 2147483647 * 2
x&& = 9223372036854775807: PRINT x&& + 1
PRINT x& * 50000&&
PRINT -(-32768)
END
