$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named MKI.
TYPE t
    mki AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.mki = 4: o.other = 2
o.mki = o.mki + o.other
PRINT o.mki; o.other
SYSTEM
