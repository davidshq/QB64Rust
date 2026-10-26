$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LINE.
TYPE t
    line AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.line = 4: o.other = 2
o.line = o.line + o.other
PRINT o.line; o.other
SYSTEM
