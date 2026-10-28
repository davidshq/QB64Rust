$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named APPEND.
TYPE t
    append AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.append = 4: o.other = 2
o.append = o.append + o.other
PRINT o.append; o.other
SYSTEM
