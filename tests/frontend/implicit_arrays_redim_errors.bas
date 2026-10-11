' TEST: check-fail
$CONSOLE:ONLY
' An array used without DIM first named in a bound of REDIM (m2-defxxx-common-arrays, review of group 5;
' verification\v24_x266, x268): the old compiler makes it dynamic, also where $DYNAMIC is not in effect, and
' allocates it as the REDIM allocates (anew on each run, or kept under _PRESERVE; no error 10). Marked "not
' supported yet". An array that exists, and one first named in a bound of DIM, are accepted. The REDIM is a
' marked declaration, so the names used without DIM after it keep the general mark (the follow-on rule).
z(1) = 2
REDIM a(z(1) + 1)
DIM b(db(1) + n)
REDIM c(rc(1) + 2)
rc(1) = 5
SYSTEM
