' TEST: typed
$CONSOLE:ONLY
' A label inside a block belongs to the enclosing body: a GOTO from outside a FOR inside a SUB reaches it, and one
' inside an IF in main is found from before the IF (m2-control-flow-slice task 5.1, 5.2;
' verification\v17_d_label_in_block_sub)
GOTO inmain
IF 1 THEN
    inmain:
    PRINT "in"
END IF
t
SYSTEM
SUB t
    GOTO inl
    FOR i = 1 TO 2
        inl:
        n = n + 1: PRINT "in"; i
        IF n > 6 THEN EXIT FOR
    NEXT
END SUB
