' TEST: check-fail
$CONSOLE:ONLY
' Array parameters (m2-defxxx-common-arrays task 7.1): the old compiler's errors as real errors (measured,
' verification\v24_x135 to x151, x155, x159, x286, x287, x291, x292). Each call stands above its SUB once and below
' it once (the calls in "below"); the lines between the errors are accepted
TYPE pt
    x AS LONG
END TYPE
TYPE other
    x AS LONG
END TYPE
DIM x(3) AS LONG, i16(3) AS INTEGER, sg(3) AS SINGLE, f3(3) AS STRING * 3, st(3) AS STRING, p(3) AS pt
DIM m(2, 4) AS LONG, y AS LONG
' "Incorrect array type passed to sub": another width, a float for an integer, a fixed-length string for a
' string and back, another length, another user type.
fill i16()
fill sg()
text f3()
fixed st()
fixed4 f3()
types p()
' "Expected arrayname()": an element, a scalar, an expression. In parentheses the old compiler's C++ build fails.
fill x(1)
fill y
fill x(1) + 1
fill (x())
' "Whole array cannot be passed to a scalar SUB parameter".
one x()
y = oneval(x())
' The number of dimensions: an index count in the SUB against the array passed, two calls that disagree, an
' array handed down a chain, a REDIM in the SUB with another number.
two x()
onedim m()
show x()
show m()
outer m()
regrow x()
' An array made by an argument has one dimension.
fill fresh&()
fresh&(1, 2) = 5
' "Name already in use" for a second array parameter of the name and type; a name with a suffix and AS.
SUB twice (a() AS LONG, a() AS LONG)
END SUB
SUB suffixed (a&() AS LONG)
END SUB
SUB fill (a() AS LONG)
    a(1) = 5
END SUB
SUB text (a() AS STRING)
END SUB
SUB fixed (a() AS STRING * 3)
END SUB
SUB fixed4 (a() AS STRING * 4)
END SUB
SUB types (a() AS other)
END SUB
SUB one (v AS LONG)
END SUB
FUNCTION oneval (v AS LONG)
END FUNCTION
SUB two (a() AS LONG)
    a(1, 2) = 5
END SUB
SUB onedim (a() AS LONG)
    a(1) = 5
END SUB
SUB show (a() AS LONG)
    PRINT UBOUND(a)
END SUB
SUB outer (a() AS LONG)
    inner a()
END SUB
SUB inner (a() AS LONG)
    a(1) = 5
END SUB
SUB regrow (a() AS LONG)
    REDIM a(2, 2) AS LONG
END SUB
' "Expected (...)": in a SUB a name that names no array is not made one by passing it.
SUB local
    fill loc()
    PRINT LBOUND(x())
END SUB
SUB late (a() AS LONG)
    a(1, 2) = 5
END SUB
SUB below
    SHARED x() AS LONG, i16() AS INTEGER, m() AS LONG, y AS LONG
    fill i16()
    fill x(1)
    fill y
    fill (x())
    one x()
    two x()
    onedim m()
    late x()
END SUB
