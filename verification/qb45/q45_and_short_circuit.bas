' QB 4.5, PDS 7.1 and QB64: is the right side of AND or OR evaluated when the left side already
' decides a condition? Found by the Wars of Napoleon port (Hutsell-Games/wars-of-napoleon,
' docs\devnotes.md, step 7): the strategic game's IF y > x AND RND > .5 (WON1.BM line 874)
' called RND in QB64 when y > x was false, and the computer's recruiting, and so every later
' RND, went another way than in the DOS game, which was built with PDS 7.1.
' Each case restarts RND with RND(-1). r1 and r2 are the first two values of that sequence: the
' RND after the case gives r1 when the case did not call RND, and r2 when it called it once.
' The FUNCTION cases count the calls of a FUNCTION instead (ncalls: CALLS is a word of PDS 7.1).
DEFINT A-Z
DECLARE FUNCTION f% (v%)
DECLARE SUB tell (what$)
COMMON SHARED r1!, r2!, ncalls
OPEN "OUT.TXT" FOR OUTPUT AS #1
d! = RND(-1): r1! = RND: r2! = RND
y = 1: x = 2

' Single-line IF.
d! = RND(-1): IF y > x AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF false AND RND"
d! = RND(-1): IF y < x AND RND > -1 THEN PRINT #1, "(then) ";
tell "IF true AND RND"
d! = RND(-1): IF y < x OR RND > .5 THEN PRINT #1, "(then) ";
tell "IF true OR RND"
d! = RND(-1): IF y > x OR RND > -1 THEN PRINT #1, "(then) ";
tell "IF false OR RND"
d! = RND(-1): IF RND > -1 AND y > x THEN PRINT #1, "(then) ";
tell "IF RND AND false"
d! = RND(-1): IF (y > x) AND (RND > .5) THEN PRINT #1, "(then) ";
tell "IF (false) AND (RND)"
d! = RND(-1): IF y > x AND 30 * RND > 15 THEN PRINT #1, "(then) ";
tell "IF false AND 30 * RND > 15"
d! = RND(-1): IF y = 1 AND x = 1 AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF true AND false AND RND"
d! = RND(-1): IF (y = 2 OR x = 1) AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF (false OR false) AND RND"
d! = RND(-1): IF y > x AND RND > .5 OR x = 2 THEN PRINT #1, "(then) ";
tell "IF false AND RND OR true"
' The right side a condition in parentheses, with RND first in it (the tactical game's NAP1.BM
' line 1352): is the whole parenthesis skipped?
d! = RND(-1): IF y > x AND (RND > .5 OR (x <> 1 AND x <> 3)) THEN PRINT #1, "(then) ";
tell "IF false AND (RND OR (true AND true))"
d! = RND(-1): IF y < x AND (RND > -1 OR (x <> 1 AND x <> 3)) THEN PRINT #1, "(then) ";
tell "IF true AND (RND OR (true AND true))"
' AND binds closer than OR: true OR (false AND RND), and false OR (true AND RND) (NAP1A.BM
' line 725).
d! = RND(-1): IF x = 2 OR y = 2 AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF true OR false AND RND"
d! = RND(-1): IF x = 3 OR y = 1 AND RND > -1 THEN PRINT #1, "(then) ";
tell "IF false OR true AND RND"
d! = RND(-1): IF x = 3 OR y = 2 AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF false OR false AND RND"
d! = RND(-1): IF NOT (y < x) AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF NOT (true) AND RND"
d! = RND(-1): IF y > x XOR RND > .5 THEN PRINT #1, "(then) ";
tell "IF false XOR RND"
d! = RND(-1): IF y AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF y AND RND (y = 1, not a relation)"
d! = RND(-1): IF y - 1 AND RND > .5 THEN PRINT #1, "(then) ";
tell "IF y - 1 AND RND (0, not a relation)"

' Block IF and ELSEIF.
d! = RND(-1)
IF y > x AND RND > .5 THEN
	PRINT #1, "(then) ";
END IF
tell "block IF false AND RND"
d! = RND(-1)
IF y > x THEN
	PRINT #1, "(then) ";
ELSEIF y > x AND RND > .5 THEN
	PRINT #1, "(elseif) ";
END IF
tell "ELSEIF false AND RND"

' Loops: the condition is false the first time, so the body never runs.
d! = RND(-1)
DO WHILE y > x AND RND > -1: PRINT #1, "(body) ";: y = y + 10: LOOP
tell "DO WHILE false AND RND"
d! = RND(-1)
DO UNTIL y < x OR RND > -1: PRINT #1, "(body) ";: y = y - 10: LOOP
tell "DO UNTIL true OR RND"
d! = RND(-1)
WHILE y > x AND RND > -1: PRINT #1, "(body) ";: y = y + 10: WEND
tell "WHILE false AND RND"
d! = RND(-1)
DO: PRINT #1, "(body) ";: LOOP WHILE y > x AND RND > -1
tell "LOOP WHILE false AND RND"

' Values, not conditions.
d! = RND(-1): z = y > x AND RND > .5
tell "z = false AND RND"
d! = RND(-1): PRINT #1, (y > x AND RND > .5);
tell "PRINT false AND RND"

' A FUNCTION on the right side.
ncalls = 0: IF y > x AND f(1) > 0 THEN PRINT #1, "(then) ";
PRINT #1, "calls"; ncalls; " IF false AND f()"
ncalls = 0: IF y < x OR f(1) > 0 THEN PRINT #1, "(then) ";
PRINT #1, "calls"; ncalls; " IF true OR f()"
ncalls = 0: IF y > x AND f(1) > 0 AND RND > .5 THEN PRINT #1, "(then) ";
PRINT #1, "calls"; ncalls; " IF false AND f() AND RND"
CLOSE #1
END

FUNCTION f (v)
ncalls = ncalls + 1
f = v
END FUNCTION

SUB tell (what$)
n! = RND
IF n! = r1! THEN
	PRINT #1, "RND not called: "; what$
ELSEIF n! = r2! THEN
	PRINT #1, "RND called:     "; what$
ELSE
	PRINT #1, "RND other:      "; what$
END IF
END SUB
