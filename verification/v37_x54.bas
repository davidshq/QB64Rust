$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a declaration with no parameters but parentheses.
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_ticks& ()
END DECLARE
PRINT h_ticks&; h_ticks&()
SYSTEM
