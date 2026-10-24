$CONSOLE
$SCREENHIDE
' Verification (m4-builtins-remaining, task 2.1): the ports libqb emulates, in screen 13: the palette through OUT &H3C8/&H3C9 and
' back through OUT &H3C7 and INP(&H3C9) (beside _PALETTECOLOR), the keyboard port &H60 with no key, a port it does
' not emulate (INP gives 0, OUT does nothing), the blink bit of port &H3C0 beside _BLINK, and a port out of range
' (error 6 for INP, nothing for OUT, which masks it).
ON ERROR GOTO handler
SCREEN 13
OUT &H3C8, 1: OUT &H3C9, 63: OUT &H3C9, 0: OUT &H3C9, 0
OUT &H3C8, 2: OUT &H3C9, 10: OUT &H3C9, 20: OUT &H3C9, 30
OUT &H3C7, 1
r = INP(&H3C9): g = INP(&H3C9): b = INP(&H3C9)
r2 = INP(&H3C9): g2 = INP(&H3C9): b2 = INP(&H3C9)
p1& = _PALETTECOLOR(1): p2& = _PALETTECOLOR(2)
k = INP(&H60): k2 = INP(&H60)
u = INP(&H379)
OUT &H378, 1
OUT &H3C0, 0: bl0 = _BLINK: OUT &H3C0, 8: bl1 = _BLINK: OUT &H3C0, 255: bl2 = _BLINK
OUT 70000, 1
OUT &H3C9, 300: OUT &H3C8, 3: OUT &H3C9, 64 + 5: OUT &H3C9, 0: OUT &H3C9, 0: p3& = _PALETTECOLOR(3)
_DEST _CONSOLE
PRINT r; g; b; r2; g2; b2; HEX$(p1&); " "; HEX$(p2&); " "; HEX$(p3&)
PRINT k = k2; u; bl0; bl1; bl2
x = INP(70000): PRINT x
x = INP(-70000): PRINT x
x = INP(65535 + 1): PRINT x
x = INP(-65536): PRINT x
PRINT "ports done"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
