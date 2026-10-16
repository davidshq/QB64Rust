$CONSOLE:ONLY
' DECLARE LIBRARY, one form: two libraries: a function of the first.
DECLARE LIBRARY "./v37c", "./v37cpp"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
PRINT c_add&(1, 2)
SYSTEM
