$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CLOSE.
TYPE t
    close AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.close = 4: o.other = 2
o.close = o.close + o.other
PRINT o.close; o.other
SYSTEM
