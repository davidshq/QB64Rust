' TEST: check-fail
$CONSOLE:ONLY
' Member arrays (m2-defxxx-common-arrays task 9.1, design D8): the old compiler's errors as real errors
' (measured, verification\v24_x182, x185-x187, x192-x198, x203, x219-x223, x225), then what is not supported
' yet, each by its own message. The TYPE blocks marked "not supported yet" stand last: a marked declaration
' drops the real errors after it (the follow-on rule, design D10 of m2-parser-breadth).
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
    g(1 TO 2, 3) AS LONG
    s(0 TO 1) AS STRING
END TYPE
' "Invalid array bounds": bounds in the wrong order; under OPTION BASE 1 the number 0 has no rule of its own.
TYPE reversed
    v(5 TO 1) AS LONG
END TYPE
OPTION BASE 1
TYPE zero
    z(0) AS LONG
END TYPE
OPTION BASE 0
' "Array bounds missing".
TYPE empty
    v() AS LONG
END TYPE
' "Cannot use _BIT inside user defined types".
TYPE bits
    b(7) AS _BIT
END TYPE
DIM t AS t1, a(2) AS t1, n AS LONG
' "Cannot change the number of elements an array has!": another number of indexes than dimensions.
t.v(1, 2) = 5
PRINT a(1).g(1)
' "Expected array index after v": a member array without an index, as a place and as a value, and before a
' further member; "Expected array index": with nothing between its parentheses (v24_x326).
t.v = 5
PRINT a(1).v
PRINT t.v.x
PRINT t.v()
' "Illegal string-number conversion".
t.v("a") = 5
' "Element is not an array": an index after a member that is a single value, and LBOUND of one (v24_x318,
' x321); "Element not defined": a member the type does not have, written like an element (x317).
PRINT t.id(1)
PRINT LBOUND(t.id)
t.m(1) = 2
' REDIM: other bounds, also the same number of elements shifted, also with _PRESERVE ("Cannot change the number
' of elements an array has!"); a bound that is not constant ("TYPE member array bounds must be numeric
' constant"); the upper bound alone without AS ("REDIM of a TYPE member array must include AS <type> or explicit
' lower TO upper bounds"); another type ("REDIM type does not match the declared TYPE member array").
REDIM t.v(0 TO 9)
REDIM t.v(1 TO 4)
REDIM _PRESERVE a(1).v(0 TO 5)
REDIM t.g(1 TO 2)
REDIM t.v(0 TO n)
REDIM t.v(3)
REDIM a(1).v(n)
REDIM t.v(0 TO 3) AS INTEGER
' The upper bound alone goes by OPTION BASE as it stands at the REDIM (v24_x323); "Unknown type" for the type
' before the names (x325).
OPTION BASE 1
REDIM t.v(3) AS LONG
OPTION BASE 0
REDIM AS LONG t.v(0 TO 3)
' "Undefined array passed to ERASE": a variable of a user type.
ERASE t
' An array argument has the parameter's kind of element, and is written with its parentheses ("Expected: Array
' Name()", v24_x319).
fill t.s()
fill t.v
' Not supported yet: a member array of an array element beside other targets of ERASE or REDIM (the old compiler
' goes on with them before an error is handled, v24_x312); an element of a member array as a target of INPUT,
' READ or the MID$ statement, and REDIM t.v(0) under OPTION BASE 1 (not measured).
ERASE a(1).v, t.v
REDIM a(1).v(0 TO 3), t.v(0 TO 3)
INPUT t.v(1)
READ a(1).v(1)
MID$(a(1).s(1), 1) = "x"
OPTION BASE 1
REDIM t.v(0) AS LONG
SYSTEM

SUB fill (b() AS LONG)
END SUB
' Marked: a bound that is not written with numbers alone (the old compiler takes a CONST and an expression of
' one, v24_x196; the blocks are read before the CONSTs here), and the _STATIC and _DYNAMIC markers
' ("_Static/_Dynamic require $Unstable:TypeFields", v24_x204, x205).
CONST top = 3
TYPE named
    v(top) AS LONG
END TYPE
TYPE marked
    v(0 TO 3) _DYNAMIC AS LONG
END TYPE
' Marked, not measured: bounds whose distance does not fit 64 bits, and a TYPE whose members together pass
' 64 bits of bytes (five members of 2^31 - 1 elements of a type of 2^31 - 1 bytes).
TYPE wide
    v(-9223372036854775807 - 1 TO 9223372036854775807) AS LONG
END TYPE
TYPE most
    v(0 TO 2147483646) AS _BYTE
END TYPE
TYPE huge
    n1(0 TO 2147483646) AS most
    n2(0 TO 2147483646) AS most
    n3(0 TO 2147483646) AS most
    n4(0 TO 2147483646) AS most
    n5(0 TO 2147483646) AS most
END TYPE
