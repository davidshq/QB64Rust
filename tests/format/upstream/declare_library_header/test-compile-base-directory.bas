$Console:Only

Declare Library "directory/fastmath"
    Function Fast_Sqrt& (ByVal val As Long)
End Declare

value = Fast_Sqrt&(2000)
Print value;
System
