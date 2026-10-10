' TEST: ir
$CONSOLE:ONLY
' DATA, READ and RESTORE in the IR (m2-builtin-statements task 4.4, design D2 and D6): the program's data is one
' list of items, each its text and whether it was quoted, with no separator byte; READ names its target places in
' order and may raise; RESTORE names an item position, with the label that gave it, and cannot raise; no
' data_offset and no libqb name appears. A DATA statement is no operation
DIM a(3) AS LONG, s AS STRING, d AS DOUBLE
DATA 1,"two"
lab:
DATA 3
READ n&, s, d
READ a(n&)
RESTORE
RESTORE lab
RESTORE done
p
SYSTEM
done:

SUB p
    DATA 4
    READ x
    RESTORE lab
END SUB
