' TEST: typed
$CONSOLE:ONLY
' An array used without DIM first named in a bound of REDIM (m4-names-implicit-arrays task 4.4, design D3): in the
' main module it is dynamic wherever it stands and is allocated in the REDIM's own mode, first in the statement;
' a DIM keeps its own mode below $DYNAMIC; in a SUB it is the SUB's implicit array, which nothing allocates
' (measured, verification\v24_x266, x268).
DIM i AS LONG
REDIM rb(rc(1) + i)
REDIM _PRESERVE rp(rq(1) + i)
'$DYNAMIC
REDIM ra(i + 1), rd(re(1) + i)
DIM dv(dw(1) + 3)
'$STATIC
s
SYSTEM

SUB s
    REDIM x(lc(1) + 4)
END SUB
