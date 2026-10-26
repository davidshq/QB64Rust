$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ELSEIF.
TYPE t
    elseif AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.elseif = 4: o.other = 2
o.elseif = o.elseif + o.other
PRINT o.elseif; o.other
SYSTEM
