$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named IMP.
TYPE t
    imp AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.imp = 4: o.other = 2
o.imp = o.imp + o.other
PRINT o.imp; o.other
SYSTEM
