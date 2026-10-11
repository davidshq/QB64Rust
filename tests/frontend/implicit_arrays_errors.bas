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
' A built-in statement's name ("Name already in use" in the old compiler): read as that statement, which is
' marked here.
cls(1) = 5
' ERASE above the first use: "Undefined array passed to ERASE".
ERASE later
later(1) = 5
' Not made an array, and marked: one leading underscore (the old compiler: "Invalid array name", unless the
' name calls a built-in the table does not know), a built-in's name with another suffix, a built-in function's
' own spelling as a place, a FUNCTION and a SUB of the name, the whole array.
_foo(1) = 2
len%(3) = 1
abs$(1) = "x"
rnd(1) = 2
f(3) = 5
PRINT LBOUND(s)
whole() = 1
' Below $DYNAMIC, a first use in the line of a block: where its allocation runs differs by block (v24_x262).
'$DYNAMIC
IF hd(1) = 0 THEN PRINT 1
DO
LOOP UNTIL hl(1) = 0
' Below $DYNAMIC, a first use in a statement that works item by item, after the part that runs first: the old
' compiler prints or reads the earlier items, or shows the prompt, before it allocates (v24_x264, x265). The
' first item of a console PRINT and the first target of READ are accepted.
PRINT pf(1); " x"; pl(1)
READ rf(1), rl(1)
WRITE wf(1)
PRINT #1, ff(1)
INPUT it(1)
LINE INPUT "p"; li$(1)
'$STATIC
' A member array beside a TYPE variable.
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
