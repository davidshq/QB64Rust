$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (51_redim_member_after_erase_static)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM s(1) AS t
ON ERROR GOTO h
ERASE s
REDIM s(1).v(2 TO 3) AS LONG
s(1).v(3) = 3
PRINT LBOUND(s(1).v); UBOUND(s(1).v); s(1).v(3)
PRINT LBOUND(s(0).v)
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
SYSTEM
