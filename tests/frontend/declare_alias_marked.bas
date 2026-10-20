' TEST: check-fail
$CONSOLE:ONLY
' `ALIAS` in a `DECLARE SUB` or `FUNCTION` outside `DECLARE LIBRARY`: the old compiler accepts it and ignores it
' (verification\v37_x08); here it is marked "not supported yet" (verification-compare), in both forms of the name.
DECLARE SUB s ALIAS t ()
DECLARE FUNCTION f& ALIAS "g" (x AS LONG)
SUB s
END SUB
FUNCTION f& (x AS LONG)
f& = x
END FUNCTION
