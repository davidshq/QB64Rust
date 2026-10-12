' TEST: typed
$CONSOLE:ONLY
' DATA, READ and RESTORE in the typed tree (m2-builtin-statements task 4.3; measured in verification\v22_c_*): the
' program's data is every DATA item in file order: the main module, a block that never runs, an included file at
' its include, the lines after SYSTEM, each procedure where it stands; bare DATA is one empty item, a quoted item
' keeps its blanks and commas, an unclosed quote runs to the line end. READ targets are places (a variable made
' on first use, an element, a member, a fixed-length string). RESTORE names a label of any body with the count of
' items before it, or nothing
TYPE rec
    n AS LONG
    f AS STRING * 4
END TYPE
DIM a(3) AS LONG, r AS rec, fx AS STRING * 4, q AS _INTEGER64
first:
DATA 1, "two" ,  three  x,,"  q, q  "
IF 0 THEN
    DATA in_if
END IF
'$INCLUDE:'inc/data.bi'
DATA
second: DATA "unclosed, to the end
READ n&, s$, made, d#
READ a(n&), r.n, r.f, fx, q
RESTORE
RESTORE first
RESTORE second
RESTORE inc_label
RESTORE in_sub
RESTORE SECOND
p
SYSTEM
DATA after system
last:

SUB p
    READ local$
    in_sub:
    DATA in a sub
    RESTORE first
    RESTORE in_sub
    RESTORE last
END SUB
