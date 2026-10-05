' TEST: typed
$CONSOLE:ONLY
' Procedure scopes (m2-procedures-and-errors D2, spec language/procedures "Procedure scopes"; measured in
' verification\v14_dim_shared_after, v14_shared_implicit, v14_shared_plain_*): which variable each name means
DECLARE SUB later (a AS LONG)
SUB before
    ' g is a local: the DIM SHARED below comes after this SUB
    g = 1
    ' SHARED with AS: main's h& (also types main's plain h from here on)
    SHARED h AS LONG
    h = 2
    ' SHARED without a type is always the SINGLE one
    SHARED m
    m = 3
END SUB
DIM SHARED g AS LONG
h = 4
x = 5
before
later g
SUB later (a AS LONG)
    ' g is main's DIM SHARED g; x is a new local; a is the parameter, plain or with its suffix
    g = a + a&
    x = 6
    DIM k AS LONG
    STATIC c AS LONG
    c = c + k
    EXIT SUB
END SUB
FUNCTION f# (v)
    ' the result, with and without the suffix; f in an expression is a recursive call
    f = v
    f# = f#(v - 1) + 1
    EXIT FUNCTION
END FUNCTION
