$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): a whole array as a value when the dynamic array is
' erased: PRINT, READ into it, SWAP of two erased arrays. Each under a handler that prints ERR.
ON ERROR GOTO handler
REDIM a(2) AS LONG
a(0) = 5
PRINT "1."; a()
ERASE a
PRINT "2."; a()
PRINT "3. after PRINT"
DATA 9
READ a()
PRINT "4. after READ"
REDIM b(2) AS LONG
ERASE b
SWAP a(), b()
PRINT "5. after SWAP"
PRINT "ok"
SYSTEM

handler:
PRINT "  error"; ERR
RESUME NEXT
