$CONSOLE:ONLY
' Verification (study 44, section 2 item 9): two libraries that each pull in test70/once, and the program names
' it too, as with the Libraries Pack's QBDS libraries. Every file starts with $INCLUDEONCE.
$USELIBRARY:'test70/user'
$USELIBRARY:'test70/once'
PRINT ONCE_K
UserSub
SYSTEM
