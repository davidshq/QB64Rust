' TEST: cpp
$CONSOLE:ONLY
' Whole values in the emitter (m2-defxxx-common-arrays task 8.2, design D9): a type without a STRING is copied in
' one move; a type that holds strings part by part, a STRING assigned, with both addresses written anew for each
' part, target first (measured, verification\v24_x306, x307: a FUNCTION in an index is called once per part). No
' copy checks for a pending error. A parameter of a user type is the address of the place passed (void*).
TYPE num
    x AS LONG
    y AS INTEGER
END TYPE
TYPE pt
    x AS LONG
    n AS STRING
    k AS num
END TYPE
DIM m AS num, ma(3) AS num, p AS pt, q AS pt, a(3) AS pt, i AS LONG
m = ma(i)
ma(one(1)) = ma(one(2))
q = p
a(one(3)) = a(one(4))
p.k = m
SWAP p, q
move q
move a(i)
bump a(i).k
SYSTEM

SUB move (p AS pt)
    p.x = p.x + 1
    bump p.k
END SUB
SUB bump (m AS num)
    m.y = m.y + 1
END SUB
FUNCTION one (n AS LONG)
    one = 1
END FUNCTION
