$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a header without a guard named by two blocks.
DECLARE LIBRARY "v37_lib/v37g"
    FUNCTION g_one& ()
END DECLARE
DECLARE LIBRARY "v37_lib/v37g"
    FUNCTION g_two& ()
END DECLARE
PRINT g_one&; g_two&
SYSTEM
