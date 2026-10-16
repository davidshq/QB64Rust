$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a name the .a does not hold (never called).
DECLARE LIBRARY "./v37c"
    FUNCTION c_none& (BYVAL v AS LONG)
END DECLARE
PRINT "ok"
SYSTEM
