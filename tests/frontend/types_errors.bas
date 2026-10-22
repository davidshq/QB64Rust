' TEST: check-fail
$CONSOLE:ONLY
' User types (m2-arrays-and-types tasks 4.1, 4.2): the old compiler's errors as real errors (verification\v18_h_*),
' and what is not supported yet: a TYPE variable as FOR variable (a TYPE inside a SUB compiles since
' m4-names-arrays: names_arrays_typed.bas; `p.x` in a SUB that does not share `p` is a plain variable of the SUB's
' own since m4-qb64pe-sources, verification\v52_x65). The TYPE blocks marked "not supported yet" stand last: a marked declaration drops
' the real errors after it (the follow-on rule of m2-parser-breadth, design D10), and a TYPE may be used above
' its block. (Whole values, parameters of a user type and STRING members: type_whole_errors.bas; member arrays:
' member_arrays_errors.bas.)
TYPE pt
    x AS LONG
END TYPE
TYPE bad2
    y& AS LONG
END TYPE
DIM p AS pt, q AS pt
DIM arr(2) AS pt
DIM nums(2) AS LONG
PRINT p
y = p + 1
y& = p
p = 5
p.c = 1
p.x% = 2
arr.x = 3
nums(1).x = 4
arr(1).c = 5
DIM p2& AS pt
FOR p = 1 TO 2: NEXT
SUB noshare
    p.x = 1
END SUB
TYPE bad1
    x&
END TYPE
