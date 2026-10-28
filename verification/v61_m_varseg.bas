$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named VARSEG.
TYPE t
    varseg AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.varseg = 4: o.other = 2
o.varseg = o.varseg + o.other
PRINT o.varseg; o.other
SYSTEM
