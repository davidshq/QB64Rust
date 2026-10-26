$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CONST.
TYPE t
    const AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.const = 4: o.other = 2
o.const = o.const + o.other
PRINT o.const; o.other
SYSTEM
