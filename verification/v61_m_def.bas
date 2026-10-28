$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DEF.
TYPE t
    def AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.def = 4: o.other = 2
o.def = o.def + o.other
PRINT o.def; o.other
SYSTEM
