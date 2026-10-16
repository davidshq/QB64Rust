$CONSOLE:ONLY
' DECLARE LIBRARY, one form: a library beside the program in a folder whose name holds a "$", which the link
' command hands to make. Recorded from such a folder, not from verification\: a folder "lib$dir" holding this
' file and a copy of v37c.a, given to the runner with --verification-root.
DECLARE LIBRARY "./v37c"
    FUNCTION c_add& (BYVAL a AS LONG, BYVAL b AS LONG)
END DECLARE
PRINT c_add&(1, 2)
SYSTEM
