' TEST: debug-info
$CONSOLE:ONLY
' A function of a DECLARE LIBRARY block has no BASIC body: neither it nor its parameter is in the debug symbol file,
' only the SUB beside it (m5-debug-build-symbols task 3.4, spec compiler/debug-info "Procedures with no body").
DECLARE LIBRARY
    FUNCTION abs_of& ALIAS "abs" (BYVAL x AS LONG)
END DECLARE
show
SUB show
    PRINT abs_of&(-3)
END SUB
