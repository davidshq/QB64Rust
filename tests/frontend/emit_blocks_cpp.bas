' TEST: cpp
$CONSOLE:ONLY
' Blocks emitted (m2-control-flow-slice task 7.1, design D9): a Skip branch is `if ((test)&&(!is_error_pending()))
' goto`, a UseValue branch (ELSEIF) a plain `if (test) goto`; a string condition goes through qbs_cleanup; a jump is
' a plain goto, guarded by is_error_pending() only after a raising operation of its statement (the FOR header with
' a raising limit); lowering labels are `L_<n>` without the event check, user labels `LABEL_<NAME>` with it. The
' FOR temporaries are static in maindata.txt, zero, of the widened types (INTEGER counts in LONG); the header
' stores all four with AssignAll
a = 2
IF a = 1 THEN
    PRINT "one"
ELSEIF CHR$(a) = "b" THEN
    PRINT "b"
ELSE
    PRINT "other"
END IF
WHILE a < 4: a = a + 1: WEND
DO
    a = a - 1
    IF a = 1 THEN EXIT DO
LOOP UNTIL a = 0
FOR i% = 1 TO INSTR(CHR$(a), "x") + 2
    again:
    PRINT i%
NEXT
IF a = 9 GOTO again
