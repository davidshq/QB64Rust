$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named TIMER.
TYPE t
    timer AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.timer = 4: o.other = 2
o.timer = o.timer + o.other
PRINT o.timer; o.other
SYSTEM
