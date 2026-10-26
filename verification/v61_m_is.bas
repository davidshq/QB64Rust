$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named IS.
TYPE t
    is AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.is = 4: o.other = 2
o.is = o.is + o.other
PRINT o.is; o.other
SYSTEM
