$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x52_underscore_sub_array)
_foo 1
_foo(1) = 2
PRINT _foo(1)
SYSTEM
SUB _foo (n)
    PRINT n
END SUB
