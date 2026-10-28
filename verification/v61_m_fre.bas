$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FRE.
TYPE t
    fre AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.fre = 4: o.other = 2
o.fre = o.fre + o.other
PRINT o.fre; o.other
SYSTEM
