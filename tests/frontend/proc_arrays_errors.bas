' TEST: check-fail
$CONSOLE:ONLY
' Arrays in procedures (m2-defxxx-common-arrays task 6.1): the old compiler's errors as real errors, one per SUB
' (measured, verification\v24_x122 to x133, x269, x281 to x283). The lines between them are accepted.
DIM g(3) AS LONG
DIM SHARED sh(3) AS LONG
' STATIC of an array the procedure has: "Name already in use" after STATIC, STATIC a(), DIM and a use without DIM.
SUB e1
    STATIC a(3) AS LONG
    STATIC a(3) AS LONG
END SUB
SUB e2
    STATIC a() AS LONG
    STATIC a() AS LONG
END SUB
SUB e3
    DIM a(3) AS LONG
    STATIC a(3) AS LONG
END SUB
SUB e4
    a(1) = 5
    STATIC a(3)
END SUB
' "Array already listed as STATIC".
SUB e5
    STATIC a() AS LONG
    STATIC a(3) AS LONG
END SUB
' "DIM/REDIM SHARED invalid within a SUB/FUNCTION".
SUB e6
    DIM SHARED a(3) AS LONG
END SUB
SUB e7
    REDIM SHARED a(3) AS LONG
END SUB
SUB e8
    DIM SHARED x AS LONG
END SUB
' SHARED a(): "Array 'a' not defined" for a main array declared below the procedure, for none at all and for
' another type than the main array's (without AS the type is the default one); "Expected ()" for bounds.
SUB e9
    SHARED below() AS LONG
END SUB
SUB e10
    SHARED none() AS LONG
END SUB
SUB e11
    SHARED g()
END SUB
SUB e12
    SHARED g(3) AS LONG
END SUB
' "Cannot change the number of elements an array has!": a listed array gets its dimensions from its first use,
' LBOUND included.
SUB e13
    STATIC y()
    PRINT LBOUND(y)
    PRINT y(1, 2)
END SUB
SUB e14
    STATIC z() AS LONG
    z(1) = 2
    REDIM z(2, 3) AS LONG
END SUB
' "Cannot redefine a static array!": in a procedure with STATIC after its header a DIM with constant bounds of a
' shared static array of the main module, and of its own static array. In any other procedure both compile (they
' raise error 10 when they run).
SUB e15 STATIC
    DIM sh(5) AS LONG
END SUB
SUB e16 STATIC
    DIM own(5) AS LONG
    DIM own(5) AS LONG
END SUB
SUB ok
    DIM sh(5) AS LONG
    DIM own(5) AS LONG
    DIM own(5) AS LONG
END SUB
' "Name already in use": DIM of a CONST name in a procedure with STATIC after its header, as in any other.
CONST kc = 1
SUB e17 STATIC
    DIM kc
END SUB
DIM below(3) AS LONG
' "STATIC must be used within a SUB/FUNCTION".
STATIC q(3) AS LONG
