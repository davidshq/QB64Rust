$CONSOLE:ONLY
' Verification: prepass label stripping (study\01 section 11.3, qb64pe.bas 2130-2131).
' A label before CONST on the same line shortens the case-preserved copy too much.
lbl1: CONST MyConstant = 42
PRINT MyConstant
SYSTEM
