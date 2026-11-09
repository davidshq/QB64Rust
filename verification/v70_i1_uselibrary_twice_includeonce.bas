$CONSOLE:ONLY
' Verification (study 44, section 2 item 9): one library named on two lines whose files start with $INCLUDEONCE.
' Without $INCLUDEONCE the library is included twice (v58_u04).
$USELIBRARY:'test70/once'
$USELIBRARY:'test70/once'
PRINT ONCE_K
OnceSub
SYSTEM
