' QB 4.5 (row 14 of the crash-class rulings, study\39): RUN from a SUB, 300 times in a row
' (QB64pe calls the main module again inside the SUB, so its stack grows with each run:
' verification\v57_r06), then RUN to a line number of the main module from a SUB (v57_r05).
DECLARE SUB s ()
DECLARE SUB t ()
DIM SHARED c AS INTEGER
c = 0
OPEN "C.TMP" FOR APPEND AS #2: CLOSE #2
OPEN "C.TMP" FOR INPUT AS #2
IF NOT EOF(2) THEN INPUT #2, c
CLOSE #2
c = c + 1
OPEN "C.TMP" FOR OUTPUT AS #2: PRINT #2, c: CLOSE #2
IF c < 300 THEN CALL s
IF c = 300 THEN CALL t
100 OPEN "OUT.TXT" FOR APPEND AS #1
PRINT #1, "at 100, run"; c
CLOSE #1
IF c > 300 THEN KILL "C.TMP": SYSTEM
CALL t

SUB s
RUN
END SUB

SUB t
c = c + 1
OPEN "C.TMP" FOR OUTPUT AS #2: PRINT #2, c: CLOSE #2
RUN 100
END SUB
