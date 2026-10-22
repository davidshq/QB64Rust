' TEST: cpp
$CONSOLE:ONLY
' STOP in C++ (m4-qb64pe-sources task 4.1): close_program=1; end(); as the old compiler writes it (qb64pe.bas
' 9918), with no exit code and no test for SUB _GL, beside SYSTEM
PRINT "a"
STOP 2
SYSTEM
