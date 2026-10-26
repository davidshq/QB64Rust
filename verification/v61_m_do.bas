$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DO.
TYPE t
    do AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.do = 4: o.other = 2
o.do = o.do + o.other
PRINT o.do; o.other
SYSTEM
