$Console:Only

t$ = "QB64 Phoenix Edition"
Print "   Text: "; t$
Print "Txt-Len: "; LTrim$(Str$(Len(t$)))
Print "-----------------------------------------"
Print "Adler32: "; Right$("00000000" + Hex$(_Adler32(t$)), 8)
Print "  Crc32: "; Right$("00000000" + Hex$(_Crc32(t$)), 8)
Print "    Md5: "; _Md5$(t$)
Print
t$ = ""
Print "   Text: "; t$
Print "Txt-Len: "; LTrim$(Str$(Len(t$)))
Print "-----------------------------------------"
Print "Adler32: "; Right$("00000000" + Hex$(_Adler32(t$)), 8)
Print "  Crc32: "; Right$("00000000" + Hex$(_Crc32(t$)), 8)
Print "    Md5: "; _Md5$(t$)

System

