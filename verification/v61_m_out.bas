$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named OUT.
TYPE t
    out AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.out = 4: o.other = 2
o.out = o.out + o.other
PRINT o.out; o.other
SYSTEM
