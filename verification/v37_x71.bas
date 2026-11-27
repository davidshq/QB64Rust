$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a SUB and a FUNCTION of one name in one block, the SUB first (Toolbox64's
' DS/HashTable.bi, HashTable_Remove). The name as a statement, after CALL, and as a value with and without its suffix.
DECLARE LIBRARY "v37_lib/v37h"
    SUB h_l (BYVAL v AS LONG)
    FUNCTION h_l& (BYVAL v AS LONG)
END DECLARE
h_l 1
CALL h_l(2)
PRINT h_l&(3)
PRINT h_l(4)
SYSTEM
