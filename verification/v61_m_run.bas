$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RUN.
TYPE t
    run AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.run = 4: o.other = 2
o.run = o.run + o.other
PRINT o.run; o.other
SYSTEM
