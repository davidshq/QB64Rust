$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named POS.
TYPE t
    pos AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.pos = 4: o.other = 2
o.pos = o.pos + o.other
PRINT o.pos; o.other
SYSTEM
