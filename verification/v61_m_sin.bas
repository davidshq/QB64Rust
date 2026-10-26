$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SIN.
TYPE t
    sin AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sin = 4: o.other = 2
o.sin = o.sin + o.other
PRINT o.sin; o.other
SYSTEM
