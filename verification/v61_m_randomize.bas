$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RANDOMIZE.
TYPE t
    randomize AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.randomize = 4: o.other = 2
o.randomize = o.randomize + o.other
PRINT o.randomize; o.other
SYSTEM
