$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DEFSNG.
TYPE t
    defsng AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.defsng = 4: o.other = 2
o.defsng = o.defsng + o.other
PRINT o.defsng; o.other
SYSTEM
