$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ENDIF.
TYPE t
    endif AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.endif = 4: o.other = 2
o.endif = o.endif + o.other
PRINT o.endif; o.other
SYSTEM
