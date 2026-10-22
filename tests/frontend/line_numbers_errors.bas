' TEST: check-fail
$CONSOLE:ONLY
' Line numbers' errors as the old compiler gives them (m4-line-numbers-whole-arrays, design D1; `verification\v16_m3_duplicate`,
' `v16_m3_goto_missing`, `v22_x86`, `v54_x01`, `x02`, `x04`, `x20`, `x21`): the same number twice (also `010` and
' `10`), a number no line has, a number of another body, RETURN n in a SUB, more than 40 characters; the numbers
' whose `last_line` the C++ build rejects (`v54_x40`, `x41`): an octal one with an 8, one above 2^64 - 1.
GOTO 99
RESTORE 98
ON ERROR GOTO 70
GOTO 12345678901234567890123456789012345678901
10 PRINT "a"
010 PRINT "b"
20 PRINT "c"
20 PRINT "d"
GOTO 08&
08& PRINT "e"
1234567890123456789012345 PRINT "f"
017& PRINT "g"
SYSTEM
SUB s
    GOSUB 70
    70 RETURN 10
END SUB
