$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RTRIM.
TYPE t
    rtrim AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.rtrim = 4: o.other = 2
o.rtrim = o.rtrim + o.other
PRINT o.rtrim; o.other
SYSTEM
