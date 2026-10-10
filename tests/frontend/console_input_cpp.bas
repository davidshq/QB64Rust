' TEST: cpp
$CONSOLE:ONLY
' Console INPUT and LINE INPUT as C++ (m2-builtin-statements task 6.4, design D5; qb64pe.bas 11123-11236 and the
' lines in study\00 section 5): qbs_print of the prompt and of "? "; per target qbs_input_variabletypes[k] (the old
' compiler's type value without its reference bits: 32 for a LONG variable, ISSTRING for a string, ISSTRING+512 for
' a LINE INPUT target) and qbs_input_variableoffsets[k] (the address, or the qbs of a string); qbs_input(count,1),
' with 0 for a ; before the prompt; if (stop_program) end(); one , after the last target is taken;
' a _BIT variable is handed over with its width and ISOFFSETINBITS (the runtime then stores nothing)
TYPE rec
    n AS LONG
    f AS STRING * 4
END TYPE
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, sg AS SINGLE, db AS DOUBLE, fl AS _FLOAT, o AS _OFFSET, uo AS _UNSIGNED _OFFSET
DIM s AS STRING, fx AS STRING * 4, a(3) AS LONG, sa(3) AS STRING, r AS rec, k AS LONG, bt AS _BIT * 3
INPUT l
INPUT "number"; l
INPUT "number: ", l
INPUT ; "stay"; l
INPUT ; l
INPUT b, ub, i, ui, l, ul
INPUT q, uq, sg, db, fl, o, uo
INPUT "three"; l, s, db
INPUT fx, a(k), sa(k), r.n, r.f
LINE INPUT s
LINE INPUT "line; "; s
LINE INPUT "line, ", s
LINE INPUT ; "stay "; s
LINE INPUT fx
LINE INPUT sa(k)
INPUT l,
LINE INPUT s,
INPUT bt
SYSTEM
