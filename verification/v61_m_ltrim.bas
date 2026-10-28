$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LTRIM.
TYPE t
    ltrim AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.ltrim = 4: o.other = 2
o.ltrim = o.ltrim + o.other
PRINT o.ltrim; o.other
SYSTEM
