' TEST: typed
$CONSOLE:ONLY
' Line numbers are labels (m4-line-numbers-whole-arrays, design D1), named as the old compiler spells them
' (`verification\v54`): `010` is `10`, `20&` is `20`; `10.5` is `10p5`, `30#` `30d`, `40!` `40s`. Each target:
' GOTO, GOSUB, RETURN n, ON n GOTO/GOSUB, THEN n, ELSE n, IF c GOTO n, RESTORE n, ON ERROR GOTO n, RESUME n,
' ON ERROR GOTO _NEWHANDLER n, an event's GOSUB n; and the same number in a SUB is a label of its own.
ON ERROR GOTO 900
ON ERROR GOTO _NEWHANDLER 900
ON TIMER(1) GOSUB 010
GOTO 10.5
010 PRINT "ten"
10.5 GOSUB 20&
ON 1 GOTO 30#, 40!
30# IF c THEN 40! ELSE 10.5
40! IF c GOTO 50
50 RESTORE 60
60 DATA 1
s
SYSTEM
20 RETURN 50
900 RESUME 50
SUB s
    10 GOTO 10
END SUB
