' TEST: parse-ok
$CONSOLE:ONLY
' IF blocks and single-line IFs in the forms the old compiler accepts (m2-parser-breadth task 6.1; measured in
' verification\v16_m4_single_if_else, v16_m4_endif_one_word, v16_m4_line_if_block, v16_m4_block_else_forms)
FOR x = 0 TO 2
    IF x = 0 THEN ' a comment after THEN keeps it a block
        PRINT "zero"
    ELSEIF x = 1 THEN PRINT "one (same line)"
    ELSE PRINT "else (same line)": PRINT "second"
    END IF
NEXT
IF 1 THEN
    PRINT "one"
ENDIF
IF 0 THEN
    PRINT "no"
ELSE IF 1 THEN
        PRINT "else if"
    END IF
END IF
IF 0 THEN
    PRINT "a": ELSE PRINT "b"
END IF
IF 1 THEN
    PRINT "c": END IF
a = 1: b = 0
IF a THEN IF b THEN PRINT "A" ELSE PRINT "B" ELSE PRINT "C"
IF a THEN IF b THEN PRINT "D" ELSE PRINT "E"
IF 1 THEN PRINT "x": PRINT "y" ELSE PRINT "z": PRINT "w"
IF a = 1 THEN : FOR p = 1 TO 3: PRINT p;: NEXT p: PRINT "done"
IF a = 0 THEN DO: PRINT "no": LOOP ELSE PRINT "else"
IF a = 1 THEN REM a comment makes this a single-line IF
IF a = 1 THEN PRINT "empty else" ELSE
IF a = 0 THEN ELSE PRINT "empty then"
IF a = 1 THEN PRINT "a" ELSE IF a = 2 THEN PRINT "b" ELSE PRINT "c"
IF 1 THEN 10 ELSE 20
10 PRINT "then 10"
IF 0 GOTO 20
20 PRINT "twenty"
SYSTEM
