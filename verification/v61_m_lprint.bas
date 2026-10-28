$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LPRINT.
TYPE t
    lprint AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.lprint = 4: o.other = 2
o.lprint = o.lprint + o.other
PRINT o.lprint; o.other
SYSTEM
