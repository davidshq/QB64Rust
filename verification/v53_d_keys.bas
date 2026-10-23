$CONSOLE:ONLY
' Verification (m4-qb64pe-builtins, task 2.1): the keyboard built-ins with no key: _KEYHIT (0), _KEYDOWN of a
' letter, of a modifier, of 0 and below (error 5), of a SINGLE code; _KEYCLEAR with no argument, with 1, 2 and 3,
' through CALL, and with 0, 4 and -1 (error 5).
ON ERROR GOTO handler
PRINT _KEYHIT; _KEYHIT
PRINT _KEYDOWN(97); _KEYDOWN(100304); _KEYDOWN(18432)
k& = 7
k& = _KEYDOWN(0)
PRINT k&
PRINT _KEYDOWN(-1)
c! = 97.4: PRINT _KEYDOWN(c!)
_KEYCLEAR
_KEYCLEAR 1
_KEYCLEAR 2
_KEYCLEAR 3
CALL _KEYCLEAR(3)
CALL _KEYCLEAR
PRINT "cleared"
_KEYCLEAR 0
_KEYCLEAR 4
_KEYCLEAR -1
PRINT "end"
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT
