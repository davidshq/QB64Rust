$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named WRITE.
TYPE t
    write AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.write = 4: o.other = 2
o.write = o.write + o.other
PRINT o.write; o.other
SYSTEM
