$CONSOLE
' Verification (m4-sub-gl, task 1.1): the OpenGL constants in a program with SUB _GL (never waited for): a hex
' value, 0xffffffff, a #define naming an earlier one (_GL_DOUBLE_EXT, _GL_LOGIC_OP, _GL_TEXTURE_COMPONENTS), one
' without GL_ (_GL_VERSION_1_1), with the suffixes &&, &, %, ~&, ! and #, their type in arithmetic (+ 1 past 32
' bits, / 2, \ 3, a product past 64 bits), stored into an INTEGER and a SINGLE, inside a SUB. (A CONST naming one is
' rejected: x43, x44.)
_DEST _CONSOLE
PRINT _GL_COLOR_BUFFER_BIT; _GL_ALL_ATTRIB_BITS; _GL_CLIENT_ALL_ATTRIB_BITS
PRINT _GL_DOUBLE; _GL_DOUBLE_EXT; _GL_LOGIC_OP; _GL_TEXTURE_COMPONENTS; _GL_VERSION_1_1; _GL_TRUE; _GL_FALSE
PRINT _GL_COLOR_BUFFER_BIT&&; _GL_CLIENT_ALL_ATTRIB_BITS&; _GL_CLIENT_ALL_ATTRIB_BITS%; _GL_CLIENT_ALL_ATTRIB_BITS~&
PRINT _GL_TRUE!; _GL_CLIENT_ALL_ATTRIB_BITS#
PRINT _GL_CLIENT_ALL_ATTRIB_BITS + 1; _GL_TRUE / 2; _GL_COLOR_BUFFER_BIT \ 3
i% = _GL_COLOR_BUFFER_BIT: s! = _GL_CLIENT_ALL_ATTRIB_BITS
PRINT i%; s!
PRINT _GL_CLIENT_ALL_ATTRIB_BITS * _GL_CLIENT_ALL_ATTRIB_BITS
inside
SYSTEM

SUB inside
    PRINT _GL_RGBA
END SUB

SUB _GL
END SUB
