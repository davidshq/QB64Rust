$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named KILL.
TYPE t
    kill AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.kill = 4: o.other = 2
o.kill = o.kill + o.other
PRINT o.kill; o.other
SYSTEM
