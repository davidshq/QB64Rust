$CONSOLE:ONLY
' Verification (QB64pe issue 95): REDIM without AS of an array of a TYPE, after REDIM ... AS the TYPE.
' QB 4.5 and PDS 7.1 reject the second REDIM with "AS clause required" (verification\qb45\q45_redim_type_no_as).
TYPE f
    foo AS INTEGER
END TYPE
ON ERROR GOTO h
REDIM a(2) AS f
REDIM a(5)
PRINT "ubound"; UBOUND(a)
a(4).foo = 2
PRINT "a(4).foo"; a(4).foo
SYSTEM
h:
PRINT "error"; ERR; "at line"; _ERRORLINE
RESUME NEXT
