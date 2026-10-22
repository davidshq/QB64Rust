' Screen program (m4-qb64pe-builtins, task 2.1): _KEYCLEAR of each buffer. For each of the letters a to d the
' program waits until _KEYHIT has seen it go down, pauses so that it is up too, empties one buffer and prints
' what is left: _KEYCLEAR 1 the INKEY$ buffer, 2 the _KEYHIT buffer, CALL _KEYCLEAR(3) the INP(&H60) buffer, and
' _KEYCLEAR with no argument all three. Then 0 and 4: error 5.
ON ERROR GOTO handler
waitfor 97
_KEYCLEAR 1
PRINT "1:"; LEN(INKEY$); _KEYHIT
waitfor 98
_KEYCLEAR 2
PRINT "2:"; _KEYHIT; INKEY$
waitfor 99
CALL _KEYCLEAR(3)
PRINT "3:"; _KEYHIT; INKEY$
waitfor 100
_KEYCLEAR
PRINT "all:"; _KEYHIT; LEN(INKEY$)
_KEYCLEAR 0
_KEYCLEAR 4
PRINT "end"
SYSTEM

handler:
PRINT "error"; ERR
RESUME NEXT

SUB waitfor (code&)
    DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
    DO: _LIMIT 500: LOOP UNTIL _KEYHIT = code&
    _DELAY .3
END SUB
