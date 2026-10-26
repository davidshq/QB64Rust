$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named OFF.
TYPE t
    off AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.off = 4: o.other = 2
o.off = o.off + o.other
PRINT o.off; o.other
SYSTEM
