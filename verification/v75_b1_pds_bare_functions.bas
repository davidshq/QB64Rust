$CONSOLE:ONLY
' Verification (SOMEDAY.md, the row on a PDS-only function read as a variable): BASIC PDS 7.1's CURDIR$, DIR$,
' STACK and SAVEPOINT can be called with no argument (verification\qb45\q45_pds_bare_functions). Without PDS
' each bare name is a variable that is never set.
PRINT "["; CURDIR$; "]"
PRINT "["; DIR$; "]"
PRINT STACK
PRINT SAVEPOINT
SYSTEM
