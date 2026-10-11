$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 4): arrays named preserve and retain in a REDIM (the
' words without their underscore).
REDIM preserve(3) AS LONG
REDIM retain(4) AS LONG
preserve(1) = 5: retain(1) = 6
PRINT UBOUND(preserve); UBOUND(retain); preserve(1); retain(1)
SYSTEM
