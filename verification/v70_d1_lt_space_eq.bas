$CONSOLE:ONLY
' Verification (study 44, section 2 item 3): `< =` with a space, as in DRAW's GUI\LAYERS.BM
' (`IF LAYER_COUNT% < = 1 THEN EXIT SUB`), in IF and in an expression.
a = 1
IF a < = 1 THEN PRINT "le"
PRINT (2 < = 1); (1 < = 1)
SYSTEM
