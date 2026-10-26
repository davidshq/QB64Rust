$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FUNCTION.
TYPE t
    function AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.function = 4: o.other = 2
o.function = o.function + o.other
PRINT o.function; o.other
SYSTEM
