$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LOC.
TYPE t
    loc AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.loc = 4: o.other = 2
o.loc = o.loc + o.other
PRINT o.loc; o.other
SYSTEM
