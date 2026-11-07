' QB 4.5, PDS 7.1 and QB64: the type of a SINGLE constant times a LONG, and SINGLE sums of the
' kind a game's casualty lines make. Found by the Wars of Napoleon port
' (Hutsell-Games/wars-of-napoleon, docs\port-study.md section 2): the DOS game wrote
' .01 * score& as .689999984577298, a DOUBLE, where QB64 writes .69.
' Part 1 writes the products with WRITE # and into a DOUBLE. Part 2 runs the game's normal()
' (twelve RNDs, less 5.5, times SQR of the variance, into an INTEGER) 200 times after
' RANDOMIZE 1#, and the casualty lines of WON.BAS that mix INTEGER and SINGLE.
DEFINT A-Z
DECLARE SUB normal (xbar%, vary%, result%)
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM s&(3)
s&(1) = 69: s&(2) = 1209: s&(3) = 123457
WRITE #1, .01 * s&(1), .01 * s&(2), .01 * s&(3)
d# = .01 * s&(1): PRINT #1, d#
x! = .01 * s&(1): PRINT #1, x!
i = .01 * s&(2): PRINT #1, i
WRITE #1, .5 * s&(1), 3 * s&(2), s&(1) / 7
RANDOMIZE 1#
l$ = ""
FOR k = 1 TO 200
CALL normal(100, 400, r)
l$ = l$ + STR$(r)
IF k MOD 20 = 0 THEN PRINT #1, l$: l$ = ""
NEXT k
l$ = ""
FOR a = 1 TO 40
killa = 3 * a + 1: killd = 7 * a + 2: x = 11 * a
killa = .8 * killa + .2 * killd
killd = .8 * killd + .2 * killa
x = .7 * x
l$ = l$ + STR$(killa) + STR$(killd) + STR$(x)
IF a MOD 10 = 0 THEN PRINT #1, l$: l$ = ""
NEXT a
CLOSE #1
SYSTEM

SUB normal (xbar, vary, result)
pct! = 0
FOR k = 1 TO 12: pct! = pct! + RND: NEXT k
pct! = pct! - 5.5
result = xbar + pct! * SQR(vary)
END SUB
