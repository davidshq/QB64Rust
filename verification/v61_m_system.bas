$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SYSTEM.
TYPE t
    system AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.system = 4: o.other = 2
o.system = o.system + o.other
PRINT o.system; o.other
SYSTEM
