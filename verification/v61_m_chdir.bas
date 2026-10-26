$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CHDIR.
TYPE t
    chdir AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.chdir = 4: o.other = 2
o.chdir = o.chdir + o.other
PRINT o.chdir; o.other
SYSTEM
