' Screen program (m4-qb64pe-builtins, task 2.1): _KEYHIT with nothing typed, then the codes of a letter, a capital,
' the up arrow, F1, Enter and the left Shift key going down and up, read until twelve have come. The INKEY$ buffer
' is emptied as the codes come, so no key of the keys file waits on it.
PRINT _KEYHIT
DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
n = 0
DO
    k& = _KEYHIT
    IF k& THEN PRINT k&;: n = n + 1
    junk$ = INKEY$
    _LIMIT 500
LOOP UNTIL n = 12
PRINT
PRINT "done"; _KEYHIT
SYSTEM
