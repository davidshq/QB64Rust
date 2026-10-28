' TEST: cpp
$CONSOLE:ONLY
' A procedure's variables in conventional memory, as the old compiler writes them (qbasic-testcases design D1-D3;
' its C++ of verification\v61_a1, p4, c4): a local is allocated from dblock in the procedure's data fragment on
' each call and given back by the epilogue's cmem_sp; a STATIC one in maindata, once. A value passed to a
' parameter whose address is taken goes through a slot passN there, placed among the body's variables in their
' order (v61_p8); VARPTR and VARSEG of the parameter are libqb's checks, as it may point into an array. CALL
' ABSOLUTE writes each argument's offset, a value's through a slot, then the call.
DIM a AS INTEGER
PRINT VARPTR(a)
t 3
DIM b AS INTEGER
PRINT VARPTR(b)
s
SYSTEM

SUB s
    DIM k AS INTEGER, l AS LONG
    STATIC st AS INTEGER
    PRINT VARPTR(k); VARSEG(k); VARPTR(l); VARPTR(st)
    t 4
    DIM v AS INTEGER
    CALL ABSOLUTE(v, 1.5!, 2, VARPTR(k))
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p); VARSEG(p)
END SUB
