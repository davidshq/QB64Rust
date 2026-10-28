$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CHR.
TYPE t
    chr AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.chr = 4: o.other = 2
o.chr = o.chr + o.other
PRINT o.chr; o.other
SYSTEM
