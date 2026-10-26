$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named MID.
TYPE t
    mid AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.mid = 4: o.other = 2
o.mid = o.mid + o.other
PRINT o.mid; o.other
SYSTEM
