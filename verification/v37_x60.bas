$CONSOLE:ONLY
' DECLARE LIBRARY, one form: 0 to a BYVAL LONG of an overloaded header function (h_over(int32_t), h_over(double)).
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION overl& ALIAS h_over (BYVAL v AS LONG)
END DECLARE
PRINT overl&(0)
SYSTEM
