$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x53_underscore_function)
PRINT _bar(1)
SYSTEM
FUNCTION _bar (n)
    _bar = n + 1
END FUNCTION
