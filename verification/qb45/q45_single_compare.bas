' QB 4.5, PDS 7.1 and QB64: an INTEGER compared with a SINGLE constant times an INTEGER. Found by the
' Wars of Napoleon port (Hutsell-Games/wars-of-napoleon, docs\devnotes.md, "Session 5's missing wait"):
' the month's update gives an army supply when IF cash(s) >= .2 * armysize(i). With 15 in cash and an
' army of 75, the product is 15.0000002 if it is kept as the x87 computed it, and 15 once rounded to a
' SINGLE. Part 1 is that case. Part 2 counts, for each multiple of 5 from 5 to 1000, whether a \ 5 >=
' .2 * a holds, with the product used at once and with it stored in a SINGLE variable first.
DEFINT A-Z
OPEN "OUT.TXT" FOR OUTPUT AS #1
c = 15: a = 75
IF c >= .2 * a THEN PRINT #1, "15 >= .2 * 75: true" ELSE PRINT #1, "15 >= .2 * 75: false"
x! = .2 * a
IF c >= x! THEN PRINT #1, "15 >= x! (x! = .2 * 75): true" ELSE PRINT #1, "15 >= x! (x! = .2 * 75): false"
direct = 0: stored = 0
FOR a = 5 TO 1000 STEP 5
	c = a \ 5
	IF c >= .2 * a THEN direct = direct + 1
	x! = .2 * a
	IF c >= x! THEN stored = stored + 1
NEXT a
PRINT #1, "of 200 multiples of 5, true at once:"; direct; " true when stored:"; stored
CLOSE #1
SYSTEM
