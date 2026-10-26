$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RND.
TYPE t
    rnd AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.rnd = 4: o.other = 2
o.rnd = o.rnd + o.other
PRINT o.rnd; o.other
SYSTEM
