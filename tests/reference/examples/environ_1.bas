$CONSOLE:ONLY
ENVIRON "QBREF_EXAMPLE=abc"
PRINT "Set: ["; ENVIRON$("QBREF_EXAMPLE"); "]"
PRINT "Not set: ["; ENVIRON$("QBREF_NO_SUCH_NAME"); "]"
PRINT "The first entry has an = in it:"; INSTR(ENVIRON$(1), "=") > 0
END
