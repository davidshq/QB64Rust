$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DRAW.
TYPE t
    draw AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.draw = 4: o.other = 2
o.draw = o.draw + o.other
PRINT o.draw; o.other
SYSTEM
