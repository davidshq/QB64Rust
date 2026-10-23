' TEST: ir
$CONSOLE:ONLY
' A numeric member of a TYPE variable as the FOR variable, through members only (m4-memory-records design D8;
' verification\v17_c_for_type_member, v56_f1, f4): the loop reads and stores the member as it would a
' variable, counting in the type wider than the member's; NEXT names it as FOR does
TYPE t
    a AS LONG
    v AS INTEGER
    s AS SINGLE
END TYPE
TYPE o
    z AS INTEGER
    n AS t
END TYPE
DIM r AS t, ro AS o
FOR r.v = 1 TO 3 STEP 2
    FOR ro.n.s = 0 TO 1 STEP 0.5: NEXT
NEXT r.v
PRINT r.v; ro.n.s
SYSTEM
