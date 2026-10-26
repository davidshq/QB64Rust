$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named GOSUB.
TYPE t
    gosub AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.gosub = 4: o.other = 2
o.gosub = o.gosub + o.other
PRINT o.gosub; o.other
SYSTEM
