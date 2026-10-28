$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PAINT.
TYPE t
    paint AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.paint = 4: o.other = 2
o.paint = o.paint + o.other
PRINT o.paint; o.other
SYSTEM
