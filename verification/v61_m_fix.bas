$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FIX.
TYPE t
    fix AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.fix = 4: o.other = 2
o.fix = o.fix + o.other
PRINT o.fix; o.other
SYSTEM
