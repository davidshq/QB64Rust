$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named GOTO.
TYPE t
    goto AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.goto = 4: o.other = 2
o.goto = o.goto + o.other
PRINT o.goto; o.other
SYSTEM
