$Console:Only
' Runtime comparison: CHR$/ASC with values > 127 (CP437/encoding)
Print Asc(Chr$(65))
Print Asc(Chr$(127))
Print Asc(Chr$(128))
Print Asc(Chr$(255))
Print Len(Chr$(128))
End
