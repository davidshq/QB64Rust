$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named OCT.
TYPE t
    oct AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.oct = 4: o.other = 2
o.oct = o.oct + o.other
PRINT o.oct; o.other
SYSTEM
