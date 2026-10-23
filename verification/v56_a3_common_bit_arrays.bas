$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): the order of conventional memory with COMMON SHARED, $DYNAMIC,
' _BIT variables (named by no VARPTR), and arrays of each kind between the scalars: of numbers (their blocks lie
' elsewhere), of strings (a descriptor per element), of fixed-length strings, of a TYPE, static and dynamic.
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
COMMON SHARED cm AS DOUBLE, cs AS STRING
DIM a AS INTEGER
DIM bt AS _BIT, bt2 AS _UNSIGNED _BIT * 3
DIM b AS INTEGER
DIM na(1 TO 5) AS LONG
DIM c AS INTEGER
DIM sa(1 TO 3) AS STRING
DIM d AS INTEGER
DIM fa(0 TO 4) AS STRING * 5
DIM ra(2) AS rec
DIM e AS INTEGER
'$DYNAMIC
DIM g AS INTEGER
DIM dn(1 TO 4) AS INTEGER
REDIM ds(1 TO 2) AS STRING
DIM h AS INTEGER
PRINT "cm"; VARPTR(cm); "cs"; VARPTR(cs)
PRINT "a"; VARPTR(a); "b"; VARPTR(b); "c"; VARPTR(c); "d"; VARPTR(d); "e"; VARPTR(e)
PRINT "g"; VARPTR(g); "h"; VARPTR(h)
PRINT "sa"; VARPTR(sa(1)); VARPTR(sa(2)); VARPTR(sa(3)); VARSEG(sa(1))
PRINT "fa"; VARPTR(fa(0)); VARPTR(fa(3)); VARSEG(fa(3)) - VARSEG(fa(0))
PRINT "ra"; VARPTR(ra(0)); VARPTR(ra(1)); VARPTR(ra(2).y); VARSEG(ra(2).y) - VARSEG(ra(0))
PRINT "na"; VARPTR(na(2)); VARSEG(na(2)) - VARSEG(fa(0))
PRINT "dn"; VARPTR(dn(2))
PRINT "ds"; VARPTR(ds(1)); VARPTR(ds(2)); VARSEG(ds(2))
REDIM ds(1 TO 3) AS STRING
PRINT "ds again"; VARPTR(ds(1)); VARPTR(ds(3))
bt = 1: bt2 = 5
PRINT bt; bt2
' Values through the offsets.
sa(2) = "hello": fa(3) = "abc": ra(1).y = 513
DEF SEG = VARSEG(sa(1))
PRINT PEEK(VARPTR(sa(2))); PEEK(VARPTR(sa(2)) + 1)
DEF SEG = VARSEG(fa(0))
PRINT PEEK(VARPTR(fa(3))); PEEK(VARPTR(fa(3)) + 4)
DEF SEG = VARSEG(ra(0))
PRINT PEEK(VARPTR(ra(1).y)); PEEK(VARPTR(ra(1).y) + 1)
DEF SEG
SYSTEM
