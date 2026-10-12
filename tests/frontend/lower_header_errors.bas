' TEST: ir
$CONSOLE:ONLY
' Header errors follow from the pending-error rule (m2-control-flow-slice task 7.3, spec compiler/pipeline; run
' end to end by corpus slice\s19_header_errors): a raising WHILE condition is a Skip branch, not taken while the
' error is pending, so the body runs; a store with a raising value is a plain Assign, made with the placeholder; a
' raising FOR limit is stored by the header's AssignAll with the start and step before the Jump to the entry, which
' is not taken while the error is pending, so the body runs with i unassigned; an ELSEIF that raises is a UseValue
' branch and tests the placeholder
DIM k AS LONG
k = -1
WHILE CHR$(k) <> "" AND n < 2
    n = n + 1
WEND
x = 5: x = INSTR(CHR$(k), "a")
FOR i = 1 TO INSTR("xyz", CHR$(k)) + 2
    PRINT i
NEXT
IF x = 2 THEN
    PRINT "two"
ELSEIF CHR$(k) = "" THEN
    PRINT "placeholder"
END IF
