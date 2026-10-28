$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named AND.
TYPE t
    and AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.and = 4: o.other = 2
o.and = o.and + o.other
PRINT o.and; o.other
SYSTEM
