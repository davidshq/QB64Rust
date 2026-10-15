' TEST: check-fail
$CONSOLE:ONLY
' DECLARE LIBRARY (m3-declare-library tasks 2.2, 2.3): the forms the old compiler rejects, each a real error with
' its reason (verification\v37_x*; the old compiler's message after each). Accepted forms: library_typed.bas.
TYPE pt
    x AS LONG
END TYPE
DECLARE DYNAMIC LIBRARY
    FUNCTION dyn_none& ()
END DECLARE
' x03 above: Expected DECLARE DYNAMIC LIBRARY "..."
DECLARE LIBRARY v37h
END DECLARE
' x05 above: Expected LIBRARY "..."
DECLARE DYNAMIC LIBRARY "./v37dll", "kernel32"
END DECLARE
' x06 above: Cannot specify multiple DYNAMIC LIBRARY names in a single DECLARE statement
DECLARE LIBRARY "v37_lib/v37h" "./v37c"
END DECLARE
' x07 above: Expected ,
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_len& (BYVAL s AS STRING)
    ' x10: STRINGs cannot be passed using BYVAL
    FUNCTION h_len2& (BYVAL s$)
    SUB h_pl (v() AS LONG)
    ' x11: Arrays cannot be passed to a library
    FUNCTION h_l& (BYVAL v AS LONG) STATIC
    ' x12: STATIC cannot be used in a library declaration
    FUNCTION h_ptsum& (p AS later)
    ' x15: Illegal SUB/FUNCTION parameter (the TYPE is defined below the block)
    FUNCTION h_lenf& (s AS STRING * 4)
    ' x26: Cannot convert type ($4) to symbol
    FUNCTION h_lenf2& (s$4)
    FUNCTION alias_at& ALIAS "h@l" (BYVAL v AS LONG)
    ' x29: Expected ALIAS name-in-library
    FUNCTION sqr# (BYVAL v AS DOUBLE)
    ' x32: Name already in use
    SUB h_tick&
    ' x44: Type symbols after a SUB name are invalid
    FUNCTION h_pt AS pt (BYVAL v AS LONG)
    ' x46: Expected (
    FUNCTION alias_none& ALIAS (BYVAL v AS LONG)
    ' x47: Expected (
    FUNCTION alias_empty& ALIAS "" (BYVAL v AS LONG)
    ' x48: Expected ALIAS name-in-library
    FUNCTION h_bit` (BYVAL v AS _BIT)
    FUNCTION h_bitp& (BYVAL v AS _BIT)
    FUNCTION h_ptv& ALIAS h_ptsum (BYVAL p AS pt)
    FUNCTION h_one& ALIAS h_l (BYVAL v AS LONG)
    FUNCTION h_ticks&
    FUNCTION h_ticks2& ALIAS h_ticks ()
    SUB h_tock
END DECLARE
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION h_one& ALIAS h_l (BYVAL v AS LONG)
    ' x40: Name already in use (a declaration in a second block)
END DECLARE
TYPE later
    x AS LONG
END TYPE

DIM p AS pt
PRINT h_bit`(0)
' x25: Name already in use (a _BIT result)
PRINT h_bitp&(0)
' x25: Name already in use (a _BIT parameter)
PRINT h_ptv&(p)
' x28: User defined types in expressions are invalid (BYVAL before a TYPE parameter)
h_one& 1
' x33: Syntax error (a library FUNCTION as a statement)
PRINT h_tock
' x34: Name already in use (a library SUB as a value)
h_one& = 3
' x41: Incorrect number of arguments passed to function
CONST c = h_one&(1)
' x50: Unexpected element
PRINT h_ticks&()
' x14: Expected (...)
PRINT h_ticks2&()
' x54: Expected (...)
PRINT h_one&("a")
' a string to a BYVAL number
SUB sub_byval (BYVAL v AS LONG)
END SUB
' x09 above: BYVAL can currently only be used with DECLARE LIBRARY
END DECLARE
' x19 above: Name already in use (DECLARE)
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION in_block& (BYVAL v AS LONG)
    PRINT "x"
END DECLARE
' x21 above: Expected SUB/FUNCTION definition or END DECLARE (#2)
DECLARE LIBRARY "v37_lib/v37h"
    FUNCTION no_end& (BYVAL v AS LONG)
' x20 below: Expected SUB/FUNCTION definition or END DECLARE (#2)
PRINT "ok"
