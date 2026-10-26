$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PEN.
TYPE t
    pen AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.pen = 4: o.other = 2
o.pen = o.pen + o.other
PRINT o.pen; o.other
SYSTEM
