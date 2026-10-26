$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named STEP.
TYPE t
    step AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.step = 4: o.other = 2
o.step = o.step + o.other
PRINT o.step; o.other
SYSTEM
