$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a SUB and a FUNCTION of one name in one block, the FUNCTION first (Toolbox64's
' Audio/OPL3X.bi, OPL3X_WriteChipRegister). The name as a statement, after CALL, and as a value.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_add& (BYVAL a AS LONG, BYVAL b AS LONG)
    SUB h_add (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
h_add 1, 2
CALL h_add(3, 4)
PRINT h_add&(5, 6)
PRINT h_add(7, 8)
SYSTEM
