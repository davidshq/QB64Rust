$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SEG.
TYPE t
    seg AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.seg = 4: o.other = 2
o.seg = o.seg + o.other
PRINT o.seg; o.other
SYSTEM
