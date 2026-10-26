$Console
$ScreenHide
_Dest _Console
ChDir _StartDir$

l = _LoadImage("./foobar.bmp", 32)

Print l; _Height(l); _Width(l);
_FreeImage l
System
