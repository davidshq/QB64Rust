$CONSOLE:ONLY
' Verification (m2-parser-breadth, M3): line numbers before a statement, before a colon, alone; GOTO a number.
10 PRINT "ten"
20 :
30
GOTO 50
40 PRINT "forty, skipped"
50 PRINT "fifty"
60 REM a remark
70 ' a comment
SYSTEM
