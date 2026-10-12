$CONSOLE:ONLY
' ERASE (spec language/arrays-and-types, "ERASE"): a static array is cleared and keeps its bounds, of every element
' type and in two dimensions; a dynamic array is released (its bounds and elements raise error 9), a second ERASE
' does nothing, and REDIM or DIM allocate it again; several names in one statement, in order; a suffix that names
' the array; a scalar of the array's name unchanged; ERASE of a shared array in a SUB. Error cases go to a handler
' that prints ERR and resumes next.
TYPE rec
    n AS LONG
    t AS STRING * 2
    d AS DOUBLE
END TYPE
ON ERROR GOTO h
DIM i AS LONG, n AS LONG
n = 4
DIM a(5) AS LONG
a(2) = 9
ERASE a
PRINT "1. static LONG: UBOUND, a(2)"; UBOUND(a); a(2)
DIM st(2) AS STRING, fx(2) AS STRING * 3, ud(2) AS rec, db(1 TO 2, 3) AS DOUBLE, b4(3) AS _BIT * 4
st(1) = "one": fx(1) = "abc": ud(1).n = 7: ud(1).t = "xy": ud(1).d = 1.5: db(2, 3) = 2.5: b4(1) = 7
ERASE st, fx, ud, db, b4
PRINT "2. static STRING, STRING * 3, a user type, DOUBLE in two dimensions, _BIT * 4: ["; st(1); "]"; LEN(st(1)); LEN(fx(1)); ASC(fx(1)); ud(1).n; ASC(ud(1).t); ud(1).d; db(2, 3); b4(1); LBOUND(db, 1); UBOUND(db, 2)
st(1) = "again": b4(1) = 3
PRINT "   and stored again: "; st(1); b4(1)
REDIM d(5) AS LONG
d(1) = 4
ERASE d
PRINT "3. dynamic: UBOUND(d)";
PRINT UBOUND(d)
PRINT "   LBOUND(d)";
PRINT LBOUND(d)
PRINT "   d(1)";
PRINT d(1)
PRINT "   d(1) = 1";
d(1) = 1
PRINT
PRINT "   ERASE d again:";
ERASE d
PRINT " nothing"
REDIM d(2) AS LONG
PRINT "   REDIM d(2): UBOUND, d(1)"; UBOUND(d); d(1)
PRINT "4. DIM z(n), ERASE z and the same DIM again, in a loop:";
FOR i = 1 TO 2
    DIM z(n + i) AS LONG
    z(1) = z(1) + i
    PRINT UBOUND(z); z(1);
    ERASE z
NEXT
PRINT
REDIM ds(2) AS STRING, df(2) AS STRING * 3, du(2) AS rec, dbt(2) AS _BIT * 4
ds(1) = "abc": df(1) = "abc": du(1).n = 7: dbt(1) = 3
ERASE ds, df, du, dbt
PRINT "5. dynamic STRING, STRING * 3, a user type, _BIT * 4: UBOUND of each";
PRINT UBOUND(ds)
PRINT UBOUND(df)
PRINT UBOUND(du)
PRINT UBOUND(dbt)
REDIM ds(1) AS STRING, df(1) AS STRING * 3, du(1) AS rec, dbt(1) AS _BIT * 4
PRINT
PRINT "   after REDIM of each: ["; ds(1); "]"; ASC(df(1)); du(1).n; dbt(1)
DIM sc&(3)
sc& = 5: sc&(1) = 6
ERASE sc&
PRINT "6. a scalar of the array's name: sc&, sc&(1)"; sc&; sc&(1)
DIM sf(3) AS LONG
sf(1) = 5
ERASE sf&
PRINT "7. the array by its suffix, ERASE sf&:"; sf(1)
REDIM m1(2) AS LONG
DIM m2(2) AS LONG
m1(1) = 1: m2(1) = 2
ERASE m1, m2, m1
PRINT "8. a dynamic and a static array in one statement, the dynamic one twice:"; m2(1); UBOUND(m2);
PRINT UBOUND(m1)
PRINT
DIM SHARED gs(3) AS LONG
REDIM SHARED gd(3) AS LONG
gs(1) = 1: gd(1) = 2
wipe
PRINT "9. ERASE of shared arrays in a SUB:"; gs(1); UBOUND(gs);
PRINT UBOUND(gd)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB wipe
    ERASE gs, gd
END SUB
