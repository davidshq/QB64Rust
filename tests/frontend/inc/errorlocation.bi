' An included file for errorlocation_cpp.bas: a statement, a DYNAMIC library and a SUB.
PRINT "in errorlocation.bi"
DECLARE DYNAMIC LIBRARY "user32"
    FUNCTION GetDesktopWindow&
END DECLARE
SUB incsub
    PRINT "incsub"
END SUB
