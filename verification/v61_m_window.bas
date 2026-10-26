$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named WINDOW.
TYPE t
    window AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.window = 4: o.other = 2
o.window = o.window + o.other
PRINT o.window; o.other
SYSTEM
