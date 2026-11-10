$INCLUDEONCE
' The top file of test70/once: a constant and a library SUB, as in the Libraries Pack's a740g/QBDS/QBDS.bi.
CONST ONCE_K = 3
DECLARE CUSTOMTYPE LIBRARY
    SUB once_copy ALIAS "memcpy" (BYVAL dst AS _UNSIGNED _OFFSET, BYVAL src AS _UNSIGNED _OFFSET, BYVAL bytes AS _UNSIGNED _OFFSET)
END DECLARE
