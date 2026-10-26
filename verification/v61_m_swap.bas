$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SWAP.
TYPE t
    swap AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.swap = 4: o.other = 2
o.swap = o.swap + o.other
PRINT o.swap; o.other
SYSTEM
