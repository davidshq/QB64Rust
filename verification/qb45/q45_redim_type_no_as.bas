' QB64pe issue 95: REDIM without AS of an array of a TYPE, after REDIM ... AS the TYPE.
' verification\v24_x327_redim_type_no_as and v24_x328 measure QB64pe.
TYPE f
    foo AS INTEGER
END TYPE
OPEN "OUT.TXT" FOR OUTPUT AS #1
ON ERROR GOTO h
REDIM a(2) AS f
REDIM a(5)
PRINT #1, "ubound"; UBOUND(a)
a(4).foo = 2
PRINT #1, "a(4).foo"; a(4).foo
CLOSE #1
END
h:
PRINT #1, "error"; ERR
RESUME NEXT
