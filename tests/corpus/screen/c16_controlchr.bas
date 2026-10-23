' Screen program (m4-qb64pe-builtins, task 2.1): _CONTROLCHR. With control characters on (the start), CHR$(12)
' clears the screen, and TAB, the cursor moves 28 to 31, CHR$(11) and CHR$(13) act; with them off, the characters
' 0 to 31 are glyphs on one row, and a bell and a line feed between two letters; the function before and after.
PRINT "gone"; CHR$(12);
PRINT _CONTROLCHR
LOCATE 5, 1: PRINT "x"; CHR$(9); "y"
LOCATE 6, 1: PRINT "ab"; CHR$(28); "c"; CHR$(29); CHR$(29); "d"
LOCATE 8, 5: PRINT "e"; CHR$(30); "f"; CHR$(31); "g"
LOCATE 10, 10: PRINT "hh"; CHR$(13); "i"
LOCATE 12, 10: PRINT "j"; CHR$(11); "k"
_CONTROLCHR OFF
LOCATE 14, 1
FOR i = 0 TO 31: PRINT CHR$(i);: NEXT
PRINT
PRINT "a" + CHR$(7) + CHR$(10) + "b"; _CONTROLCHR
_CONTROLCHR ON
PRINT _CONTROLCHR
SYSTEM
