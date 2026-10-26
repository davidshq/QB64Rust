
Const chunkSIZEOF% = 4 + 4

Const CHformLEN% = 4
Const formSIZEOF% = chunkSIZEOF% + CHformLEN% 'remove trailing % and it works

Const CHthdrLEN% = 30 + 2 + 4
Const thdrSIZEOF% = chunkSIZEOF% + CHthdrLEN% 'no error

Const CHcsetLEN% = 16 + 264 + 2 + 2 + 2 + 2
Const csetSIZEOF% = chunkSIZEOF% + CHcsetLEN% 'no error

Const CHwposLEN% = 30 + 2 + 2
Const wposSIZEOF% = chunkSIZEOF% + CHwposLEN% 'no error

Const CHvarsLEN% = 4 + 2 + 2
Const varsSIZEOF% = chunkSIZEOF% + CHvarsLEN% 'no error

Const CHtlogLEN% = 30 + 12 + 80
Const tlogSIZEOF% = chunkSIZEOF% + CHtlogLEN% 'no error
