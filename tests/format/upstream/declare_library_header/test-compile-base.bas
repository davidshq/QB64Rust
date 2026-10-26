$Console:Only

' This location is relative to the source file, not QB64-PE or the location
' QB64-PE was run from.
Declare Library "fastmath"
    Function Fast_Sqrt& (ByVal val As Long)
End Declare

value = Fast_Sqrt&(2000)
Print value;
System
