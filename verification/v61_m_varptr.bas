$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named VARPTR.
TYPE t
    varptr AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.varptr = 4: o.other = 2
o.varptr = o.varptr + o.other
PRINT o.varptr; o.other
SYSTEM
