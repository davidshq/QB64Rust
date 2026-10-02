$CONSOLE:ONLY
' Verification: which division-by-zero cases ON ERROR can trap.
' Claims: study\03 section 2.10, study\02 lines 827 and 929, study\08.
ON ERROR GOTO handler

PRINT "== float / zero =="
z! = 0: s! = 1 / z!: PRINT s!
z# = 0: d# = 1 / z#: PRINT d#
PRINT 1 / 0

PRINT "== CINT overflow (error 6 expected) =="
x% = CINT(40000): PRINT x%

PRINT "== MOD by zero =="
z% = 0: x% = 7 MOD z%: PRINT x%

PRINT "== integer \ zero =="
z% = 0: x% = 7 \ z%: PRINT x%

PRINT "== reached end =="
SYSTEM

handler:
PRINT "trapped error"; ERR
RESUME NEXT
