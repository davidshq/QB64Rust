' TEST: check-fail
$CONSOLE:ONLY
' Arrays used without DIM (m2-defxxx-common-arrays task 5.1): the old compiler's errors as real errors
' (verification\v24_x114 to x118, x261, v18_d_index_string), then the names design D5 keeps from becoming an array,
' each still with a "not supported yet" mark of its own, then what waits for a later group. The lines between
' them are accepted.
z(1) = 1
' Another number of indexes: "Cannot change the number of elements an array has!", also after LBOUND.
z(1, 2) = 1
PRINT LBOUND(lb)
lb(1, 2) = 3
' DIM of an array that was used: "Cannot redefine a static array!", also after LBOUND.
DIM z(5)
PRINT UBOUND(ub)
DIM ub(5)
' A string as an index: "Illegal string-number conversion".
z("a") = 1
' A built-in statement's name: "Name already in use" as in the old compiler (m3-window-output task 3.2; the
' parser reads the line as that statement, `sema` sees the shape of an assignment).
cls(1) = 5
' ERASE above the first use: "Undefined array passed to ERASE".
ERASE later
later(1) = 5
' Not made an array: one leading underscore (the old compiler: "Invalid array name"), a built-in's name with
' another suffix, a built-in function's own spelling as a place ("Expected variable ="), a FUNCTION and a SUB of
' the name (verification\v24_x106, v55_x83: "Name already in use"); the whole array (marked).
_foo(1) = 2
len%(3) = 1
abs$(1) = "x"
rnd(1) = 2
f(3) = 5
PRINT LBOUND(s)
whole() = 1
' A name written like an element of a member array, of a member the type does not have: "Element not defined".
TYPE pt
    x AS LONG
END TYPE
DIM v AS pt
v.m(1) = 2
SYSTEM
FUNCTION f (n)
    f = n
END FUNCTION
SUB s
END SUB
