// Fill out dependency macros
#if !defined(DEPENDENCY_NO_SOCKETS) && !defined(DEPENDENCY_SOCKETS)
#    define DEPENDENCY_SOCKETS
#endif

#if !defined(DEPENDENCY_NO_PRINTER) && !defined(DEPENDENCY_PRINTER)
#    define DEPENDENCY_PRINTER
#endif

#if !defined(DEPENDENCY_NO_ICON) && !defined(DEPENDENCY_ICON)
#    define DEPENDENCY_ICON
#endif

#if !defined(DEPENDENCY_NO_SCREENIMAGE) && !defined(DEPENDENCY_SCREENIMAGE)
#    define DEPENDENCY_SCREENIMAGE
#endif

#ifndef INC_COMMON_CPP
#    define INC_COMMON_CPP
#    include "os.h"

#    ifndef DEPENDENCY_CONSOLE_ONLY
#        define QB64_GUI
#    endif

#    ifdef QB64_GUI
#        include <glad/gl.h>
#        ifdef QB64_MACOSX
#            include <OpenGL/gl.h>
#            include <OpenGL/glext.h>
#            include <OpenGL/glu.h>
#        else
#            include <GL/gl.h>
#            include <GL/glext.h>
#            include <GL/glu.h>
#        endif
#        ifdef QB64_EMSCRIPTEN
#            include <cstdint>
#            include <cstdlib>
static inline void qb64rust_glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border,
                                         GLenum format, GLenum type, const void *pixels) {
    while (glad_glGetError() != GL_NO_ERROR) {
    }
    if (format == GL_BGRA && type == GL_UNSIGNED_BYTE && pixels != nullptr) {
        size_t count = (size_t)width * (size_t)height;
        uint8_t *rgba = (uint8_t *)malloc(count * 4);
        const uint8_t *bgra = (const uint8_t *)pixels;
        for (size_t i = 0; i < count; i++) {
            rgba[i * 4] = bgra[i * 4 + 2];
            rgba[i * 4 + 1] = bgra[i * 4 + 1];
            rgba[i * 4 + 2] = bgra[i * 4];
            rgba[i * 4 + 3] = bgra[i * 4 + 3];
        }
        glad_glTexImage2D(target, level, GL_RGBA, width, height, border, GL_RGBA, type, rgba);
        free(rgba);
    } else {
        glad_glTexImage2D(target, level, internalformat == GL_BGRA ? GL_RGBA : internalformat, width, height, border,
                          format == GL_BGRA ? GL_RGBA : format, type, pixels);
    }
}
static inline GLint qb64rust_gluBuild2DMipmaps(GLenum target, GLint internalformat, GLsizei width, GLsizei height, GLenum format,
                                               GLenum type, const void *pixels) {
    qb64rust_glTexImage2D(target, 0, internalformat, width, height, 0, format, type, pixels);
    return 0;
}
#            undef glTexImage2D
#            define glTexImage2D qb64rust_glTexImage2D
#            define gluBuild2DMipmaps qb64rust_gluBuild2DMipmaps
#        endif
#    endif

#    ifdef QB64_WINDOWS

#        include <winsock2.h>

#        ifndef QB64_GUI
#            undef int64 // definition of int64 from os.h conflicts with a definition within windows.h, temporarily undefine then redefine
#            include <windows.h>
#            define int64 __int64
#        endif

#        include <shfolder.h>

#        include <float.h>
#        include <winbase.h>

#    endif

// common includes
#    include <cmath>
#    include <errno.h>
#    include <fcntl.h>
#    include <fstream>
#    include <inttypes.h>
#    include <iostream>
#    include <limits.h>
#    include <stdint.h>
#    include <stdio.h>
#    include <string.h>
#    include <time.h>

// OS/compiler specific includes
#    ifdef QB64_WINDOWS
#        include <direct.h>
#        ifdef DEPENDENCY_PRINTER
#            include <winspool.h>
#        endif
#        include <csignal>
#        include <process.h> //required for multi-threading
#        if defined DEPENDENCY_AUDIO_MINIAUDIO || defined QB64_GUI
#            include <mmsystem.h>
#        endif

#    else

#        include <dlfcn.h>
#        include <pthread.h>
#        include <stdlib.h>
#        include <sys/stat.h>
#        include <sys/types.h>
#        include <sys/wait.h>
#        include <unistd.h>

#    endif

#    include "error_handle.h"
#    include "graphics.h"
#    include "qbs.h"

// QB64 internal variable type flags (internally referenced by some C functions)
#    define ISSTRING 1073741824
#    define ISFLOAT 536870912
#    define ISUNSIGNED 268435456
#    define ISPOINTER 134217728
#    define ISFIXEDLENGTH 67108864 // only set for strings with pointer flag
#    define ISINCONVENTIONALMEMORY 33554432
#    define ISOFFSETINBITS 16777216

struct ontimer_struct {
    uint8 allocated;
    uint32 id;        // the event ID to trigger (0=no event)
    int64 pass;       // the value to pass to the triggered event (only applicable to ON ... CALL ...(x)
    uint8 active;     // 0=OFF, 1=ON, 2=STOP
    uint8 state;      // 0=untriggered,1=triggered
    double seconds;   // how many seconds between events
    double last_time; // the last time this event was triggered
};

struct byte_element_struct {
    uint64 offset;
    int32 length;
};

#    include "memblock.h"

#    ifndef QB64_WINDOWS
static inline void ZeroMemory(void *ptr, size_t bytes) {
    ::memset(ptr, 0, bytes);
}
#    endif

#endif // INC_COMMON_CPP
