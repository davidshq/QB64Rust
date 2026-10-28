$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ASC.
TYPE t
    asc AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.asc = 4: o.other = 2
o.asc = o.asc + o.other
PRINT o.asc; o.other
SYSTEM
