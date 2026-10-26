$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named _MINE.
TYPE t
    _mine AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o._mine = 4: o.other = 2
o._mine = o._mine + o.other
PRINT o._mine; o.other
SYSTEM
