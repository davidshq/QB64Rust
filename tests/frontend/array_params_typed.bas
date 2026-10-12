' TEST: typed
$CONSOLE:ONLY
' Array parameters (m2-defxxx-common-arrays task 7.1, design D6; measured, verification\v24_g_params, v24_x135 to
' x160, x178 to x180, x286 to x296): a parameter written name() is an array of the procedure, of any element type
' an array may have (a _BIT and a user type too); what stands between its parentheses is not read. Its argument is
' a whole array written name(): static, dynamic, the procedure's own parameter passed on, a listed array, or in
' the main module a name that names no array yet (an array of one dimension, 0 to 10). The element types have one
' kind and width; signedness and _OFFSET beside _INTEGER64 are not compared. The number of dimensions of a
' parameter comes from everything that names it: indexes, a DIM or REDIM of it, and every array passed, above or
' below the procedure (show, called before and defined after; down, which passes it on; LBOUND and UBOUND say
' nothing). ERASE of a parameter is an EraseParam. A scalar parameter of the same name is another variable
TYPE pt
    x AS LONG
END TYPE
DIM x(1 TO 3) AS LONG, u(3) AS _UNSIGNED LONG, m(2, 4) AS LONG, s(2) AS STRING, f(2) AS STRING * 3
DIM p(2) AS pt, b(3) AS _BIT * 4, o(3) AS _INTEGER64
REDIM d(2) AS LONG
COMMON c() AS LONG
show m()
fill x()
fill u()
fill d()
grow d()
texts s(), f(), p(), b(), o()
PRINT pick(x(), 2)
CALL fill(x())
fill never&()
down m()
make c()
c(1, 2) = 3
both x(), 9
skipped x(), x(), m()
SYSTEM
SUB fill (a() AS LONG)
    DIM i AS LONG
    FOR i = LBOUND(a) TO UBOUND(a)
        a(i) = i * 2
    NEXT
END SUB
SUB grow (a() AS LONG)
    REDIM _PRESERVE a(10) AS LONG
    ERASE a
    DIM a(5) AS LONG
END SUB
SUB texts (a() AS STRING, b() AS STRING * 3, c() AS pt, e(,) AS _BIT * 4, g(1) AS _OFFSET)
    a(1) = b(1)
    c(2).x = e(1) + g(1)
END SUB
FUNCTION pick (a() AS LONG, i AS LONG)
    pick = a(i)
END FUNCTION
SUB show (m() AS LONG)
    PRINT UBOUND(m, 2)
END SUB
SUB down (a() AS LONG)
    show a()
END SUB
SUB make (a() AS LONG)
    REDIM a(3, 4) AS LONG
END SUB
SUB both (a() AS LONG, a AS LONG)
    PRINT a(1); a
END SUB
SUB unused (a() AS LONG, z())
    PRINT LBOUND(a)
    unused a(), z()
END SUB
' Anything may stand between the parentheses and is skipped (measured, v24_x303).
SUB skipped (a(n + 1, "x") AS LONG, b&(-1), c(1 TO 5) AS LONG)
    PRINT a(1); b&(1); c(1, 2)
END SUB
