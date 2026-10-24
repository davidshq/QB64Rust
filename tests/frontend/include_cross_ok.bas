' TEST: check-ok
$CONSOLE:ONLY
' Blocks, SUBs and FUNCTIONs opened in one file and closed in another, both ways, as the old compiler reads
' included text (m4-statements-metacommands, design D8; verification\v16_m7_block_across, v16_m7_sub_across,
' v57_b01-b20).
' A FOR closed by NEXT in an included file.
FOR i = 1 TO 2
'$INCLUDE:'inc/next.bi'
' A FOR opened in an included file and closed here, with its variable.
'$INCLUDE:'inc/cross_for_open.bi'
    PRINT "body"; k
NEXT k
' An IF whose ELSE is in an included file.
IF i = 1 THEN
    PRINT "one"
'$INCLUDE:'inc/cross_else.bi'
    PRINT "not one"
END IF
' NEXT j, i in an included file closes two loops of this file.
FOR i = 1 TO 2
    FOR j = 1 TO 2
'$INCLUDE:'inc/cross_next_two.bi'
s
PRINT f(2)
SYSTEM
' A SUB closed by END SUB in an included file, whose later lines are the main module's again.
SUB s
'$INCLUDE:'inc/cross_endsub.bi'
' A FUNCTION opened in an included file and closed here.
'$INCLUDE:'inc/cross_function_open.bi'
    f = x * 3
END FUNCTION
