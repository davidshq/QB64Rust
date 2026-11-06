$CONSOLE:ONLY
' v68 "Unused variable" for shared variables: DIM SHARED, COMMON SHARED and SHARED in a SUB, each never used again.
COMMON SHARED commonone AS INTEGER
DIM SHARED dimshared AS LONG
DIM SHARED usedinsub AS LONG
mainvar = 3
UsesShared
SYSTEM

SUB UsesShared
    SHARED mainvar
    usedinsub = 1
    PRINT usedinsub
END SUB
