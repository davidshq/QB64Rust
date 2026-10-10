' TEST: ir
$CONSOLE:ONLY
' Console INPUT and LINE INPUT in the IR (m2-builtin-statements task 6.4, design D2): the source is the console
' with the prompt's bytes when one is written, "question" where ? and a blank follow it (an INPUT without a prompt
' or with ; after it, never a LINE INPUT) and "stay" for a ; before the prompt; then the target places in order.
' No qbs_input, no type code and no address appears; each may raise
TYPE rec
    n AS LONG
    f AS STRING * 4
END TYPE
DIM l AS LONG, d AS DOUBLE, s AS STRING, fx AS STRING * 4, a(3) AS LONG, r AS rec, k AS LONG
INPUT l
INPUT "number"; l
INPUT "number: ", l
INPUT ; "stay"; l
INPUT ; l
INPUT "three"; l, s, d
INPUT fx, a(k), r.n, r.f
INPUT made, made$
LINE INPUT s
LINE INPUT "line; "; s
LINE INPUT "line, ", s
LINE INPUT ; "stay "; s
LINE INPUT fx
p
SYSTEM

SUB p
    DIM q AS _INTEGER64, t AS STRING
    INPUT "in a sub"; q
    LINE INPUT t
END SUB
