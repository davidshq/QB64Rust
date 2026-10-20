' TEST: check-fail
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' DIM with bounds of a name whose part before the dot is a TYPE variable: only DIM of a member array without a
' suffix was measured ("DIM: Expected ,", verification\v35_x37, in typefields_rejected.bas). A name that is no
' member, a member that is no array and a suffix were not, and are marked "not supported yet"
' (verification-compare).
TYPE t
    v(1) _DYNAMIC AS LONG
    n AS LONG
END TYPE
DIM x AS t
DIM x.nope(3)
DIM x.n(3)
DIM x.v&(3)
