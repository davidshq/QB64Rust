$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CASE.
TYPE t
    case AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.case = 4: o.other = 2
o.case = o.case + o.other
PRINT o.case; o.other
SYSTEM
