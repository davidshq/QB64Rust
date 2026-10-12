' TEST: typed
$CONSOLE:ONLY
' Whole values and parameters of a user type (m2-defxxx-common-arrays task 8.2; measured, verification\v24_h_whole,
' v24_h_type_params, v24_x306-x308): q = p between variables, elements and members of one TYPE, in every
' direction and to itself; SWAP of two such places; a parameter AS a TYPE given a variable, an element, a member
' and the procedure's own parameter, in a SUB and a FUNCTION.
TYPE pt
    x AS LONG
    n AS STRING
END TYPE
TYPE box
    a AS pt
    b AS pt
END TYPE
DIM p AS pt, q AS pt, a(3) AS pt, bx AS box, bs(2) AS box, i AS LONG
q = p
q = a(2)
a(1) = p
a(i + 1) = a(i)
p = p
bx.a = p
bx.b = bx.a
q = bs(i).b
bs(1).a = a(2)
SWAP p, q
SWAP a(1), a(2)
move q
move a(2)
move bx.a
move bs(1).b
PRINT twice(q); twice(a(i))
passon q
SYSTEM

SUB move (p AS pt)
    p.x = p.x + 1
    p.n = p.n + "m"
END SUB
FUNCTION twice (p AS pt)
    twice = p.x * 2
END FUNCTION
SUB passon (p AS pt)
    DIM l AS pt
    move p
    l = p
    p = l
END SUB
