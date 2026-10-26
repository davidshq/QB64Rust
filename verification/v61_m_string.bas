$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named STRING.
TYPE t
    string AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.string = 4: o.other = 2
o.string = o.string + o.other
PRINT o.string; o.other
SYSTEM
