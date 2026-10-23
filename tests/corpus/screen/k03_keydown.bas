' Screen program (m4-qb64pe-builtins, task 2.1): _KEYDOWN of a held key. The left Shift key: the program waits
' until it is down, prints it, waits until it is up and prints it again, then the _KEYHIT codes it left. The
' space bar: down (the left Shift key not), up, and what INKEY$ got.
announce
DO: _LIMIT 500: LOOP UNTIL _KEYDOWN(100304)
PRINT _KEYDOWN(100304);
DO: _LIMIT 500: LOOP WHILE _KEYDOWN(100304)
PRINT _KEYDOWN(100304)
PRINT _KEYHIT; _KEYHIT; _KEYHIT
announce
DO: _LIMIT 500: LOOP UNTIL _KEYDOWN(32)
PRINT _KEYDOWN(32); _KEYDOWN(100304);
DO: _LIMIT 500: LOOP WHILE _KEYDOWN(32)
PRINT _KEYDOWN(32)
PRINT "["; INKEY$; "]"
SYSTEM

SUB announce
    DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
END SUB
