$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named INPUT.
TYPE t
    input AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.input = 4: o.other = 2
o.input = o.input + o.other
PRINT o.input; o.other
SYSTEM
