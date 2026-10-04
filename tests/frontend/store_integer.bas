' TEST: typed
$CONSOLE:ONLY
' numeric-semantics: storing into an integer variable
x% = 2.5
x% = 3.5
d# = 2.5000001: x% = d#: l& = d#
x% = 70000
q&& = 2.5#
s! = 7
END
