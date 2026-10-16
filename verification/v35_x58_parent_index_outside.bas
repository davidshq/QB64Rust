$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (58_parent_index_outside)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM p(2) AS t
ON ERROR GOTO h
PRINT "1.";: REDIM p(9).v(4 TO 6) AS LONG: PRINT UBOUND(p(0).v); UBOUND(p(1).v)
PRINT "2.";: PRINT UBOUND(p(9).v)
PRINT "3.";: p(9).v(5) = 55: PRINT p(0).v(5)
PRINT "4.";: PRINT p(9).v(5)
PRINT "5.";: ERASE p(9).v: PRINT UBOUND(p(0).v)
PRINT "6.";: PRINT LBOUND(p(1).v)
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
SYSTEM
