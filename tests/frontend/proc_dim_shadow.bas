' TEST: typed
$CONSOLE:ONLY
' A local DIM shadows a DIM SHARED variable of the same name (m2-procedures-and-errors D2, measured in
' verification\v14_dim_local_plain, _same, _other): plain g, then g&, mean the local or the shared one
DIM SHARED g AS LONG
SUB plain
    ' plain g is now the local g!; g& is still main's
    DIM g
    g = 2.5
    g& = 1
END SUB
SUB same
    ' g and g& are the local LONG
    DIM g AS LONG
    g = 3
    g& = 4
END SUB
SUB other
    ' plain g is the local STRING; g& is still main's
    DIM g AS STRING
    g = "x"
    g& = 5
END SUB
