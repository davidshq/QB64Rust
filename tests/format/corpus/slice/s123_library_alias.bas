$Console:Only
' DECLARE LIBRARY with a header (v37h.h in this program's .files folder): ALIAS as a string and as a
' name, C++ names (a template, a namespace) and an overload chosen by the C++ type of the argument
' (verification\v37_e, x60).
Declare Library "./v37h"
    Function maxl& Alias "h_max<int32_t>" (ByVal a As Long, ByVal b As Long)
    Function maxd# Alias "h_max<double>" (ByVal a As Double, ByVal b As Double)
    Function twice& Alias "hns::twice" (ByVal v As Long)
    Function overl& Alias h_over (ByVal v As Long)
    Function overd& Alias "h_over" (ByVal v As Double)
    Function addl& Alias h_add (ByVal a As Long, ByVal b As Long)
End Declare

Print "template:"; maxl&(3, 9); maxd#(2.5, -1)
Print "namespace:"; twice&(21)
x# = 1: y& = 1
Print "overload:"; overl&(0); overl&(1); overd&(1); overd&(1.5); overd&(x#); overd&(y&); overd&(y& + 0.5)
Print "alias name:"; addl&(40, 2)
System
