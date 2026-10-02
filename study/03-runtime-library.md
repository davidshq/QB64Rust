# 03 — QB64pe C++ Runtime Library Study

Repo studied: `C:\code\qb64-new\QB64pe` (HEAD `16f62978`, "Automatic update of ./internal/source", 2026-09-30). Read-only study.
All paths below are relative to `internal\c\` unless stated. `libqb.cpp:N` means line N of `internal\c\libqb.cpp`.

**Coverage honesty note.** Read in full: `qbx.cpp`, `common.h`, `os.h`, most `libqb\include\*.h`, `qbs.cpp` (1-420), `qbs_cmem.cpp`, `error_handle.cpp`, `main-thread-*.cpp`, `threading-windows.cpp`, `threading.cpp`, all `build.mk` files. `libqb.cpp` (27,119 lines): a complete function outline was extracted (all ~297 top-level definitions) and roughly 4,000 lines were read directly (startup/main loop, display pipeline head/tail, GL render request, image core, conventional memory, CPU/interrupt emulation, OUT/INP/WAIT, RND, OPEN, END/RUN, SLEEP, start of PRINT / PRINT USING / SCREEN / _PUTIMAGE). The bodies of the big drawing/printing/input routines (PAINT, CIRCLE, DRAW, GET/PUT, `qbs_input`, `print_using`, `printchr`, text-mode renderer middle, `sub__screenprint`, hardware_img_* internals) were **sampled, not read line by line**. Other `libqb\src` modules were outlined by function signature with selective reads. Vendored code in `parts\` was identified from headers and makefiles only.

---

## 0. Top-level shape

| Piece | Size | Role |
|---|---|---|
| `libqb.cpp` | 27,119 lines / 1.1 MB, ~297 functions | The monolith: graphics, text, file statements, INPUT/PRINT, emulation, TCP, display, `main()` |
| `qbx.cpp` | 1,608 lines | The translation unit the *user program* is compiled into. `#include`s generated `..\temp\*.txt` fragments; owns `QBMAIN`, `TIMERTHREAD`, `evnt()`, `events()`, CHAIN, device API, ON TIMER/STRIG, `cmem[]` |
| `common.h` | 129 lines | Dependency macro defaults, GL/OS includes, type-flag constants, `ontimer_struct`, `byte_element_struct` |
| `os.h` | 27 lines | `int32`/`uint8`/`ptrszint` macro typedefs, `ptrsz` |
| `libqb\include` | 49 headers | Modularised API |
| `libqb\src` | 37 .cpp + `logging\` | Modularised implementation (~24.5k lines) |
| `parts\` | 8 dirs | First-party glue + vendored third-party libs |

Important: the windowing backend is **GLFW 3.5.1 + GLAD 2.0.8**, hidden behind a first-party "GLUT emulation" class (`libqb\src\glut-emu.cpp`, 2,665 lines). All the `GLUT_*` names are historical; FreeGLUT is gone. Comments still say "FreeGLUT" (`libqb\src\main-thread-gui.cpp:29`).

`libqb.cpp` is compiled per program (flags vary by `DEPENDENCY_*`), `libqb\src\*.o` are compiled once with `-O3 -Wall -Wextra` (`libqb\build.mk:57`).

---

## 1. Process / thread architecture

### 1.1 Threads

| Thread | Entry | Created at | Does |
|---|---|---|---|
| **Initial (OS main) thread** | `main()` `libqb.cpp:25807` → `libqb_start_main_thread()` → `GLUTEmu_MainLoop()` | process start | Owns the GLFW window and GL context. Pumps OS events, runs keyboard/mouse callbacks, idle func (frame pacing), `GLUT_DISPLAY_REQUEST` (all GL rendering), and calls the user's `SUB _GL`. In console-only builds it instead runs `MAIN_LOOP` directly. |
| **QBMAIN thread** | `QBMAIN(void*)` `qbx.cpp:1562` | `libqb.cpp:26108-26109` | Runs the compiled BASIC program (`#include "../temp/main.txt"`, `qbx.cpp:1606`). All `sub_*`/`func_*` calls happen here (except those invoked from `SUB _GL`). |
| **Timer thread** | `TIMERTHREAD(void*)` `qbx.cpp:1361` | `libqb.cpp:26111-26112` | 1 ms loop; marks `ontimer[i].state=1` and sets `qbevent=1` when an ON TIMER interval elapses. |
| **MAIN_LOOP thread** ("display/housekeeping thread") | `MAIN_LOOP(void*)` `libqb.cpp:26124` | `main-thread-gui.cpp:126-127` | ~16 ms loop: gamepad poll, BIOS tick emulation in `cmem[0x46c]`, `snd_update()`, LPRINT flush, and every second iteration `display()` (software frame build, ~31 fps). Also the shutdown orchestrator. |
| **Audio thread(s)** | internal to miniaudio | `AudioEngine::Initialize` `parts\audio\audio.cpp:2511` (lazy) | miniaudio device callback; pulls from `RawStream` queues guarded by `libqb_mutex` (`audio.cpp:401-480`). |
| **HTTP thread** | `libqb_curl_thread_handler` `libqb\src\qb_http.cpp:249` | `libqb_http_init()` `qb_http.cpp:509-515`, called from `main()` `libqb.cpp:26104` | curl-multi polling loop. |

Thread primitives are a thin first-party wrapper: `libqb\include\thread.h`, `mutex.h`, `condvar.h`, `completion.h`; implementations `threading-windows.cpp` (`_beginthreadex`, `CRITICAL_SECTION`, `CONDITION_VARIABLE`) and `threading-posix.cpp` (pthreads).

### 1.2 Startup sequence (`main()`, `libqb.cpp:25807-26121`)

1. `clock_init()`; Windows: attach to parent console unless stdout is an msys/cygwin pipe (`25810-25821`, `isValidCygwinPipe` `25774`).
2. `libqb_log_init()`; Linux/X11: `XInitThreads()` (`25826`).
3. Reset render state and the 3 `display_frame[]` slots (`25838-25856`).
4. `set_dynamic_info()` (`qbx.cpp:506`, body is generated `temp\dyninfo.txt`) — sets compile-time-chosen globals such as `ScreenResize`, `console`, `screen_hide_startup`, `asserts`, `vwatch`.
5. Create handle lists (`25867-25872`); `hardware_img_handles` is the only "threadsafe" list.
6. Default ON KEY bindings, timer 0, built-in font metrics for fonts 8/14/16 (+1 = double width) (`25886-25910`).
7. Image table: indexes 0 and 1 reserved/invalid (`25912-25916`).
8. Emulated CPU register tables (`25918-25954`).
9. `mem_static` 1 MB arena, zero `cmem[]`, `dblock = &cmem + 1280` (0:500h), allocate `nothingvalue`, `nothingstring`, `singlespace` in cmem (`25959-25976`).
10. Save start dir, **`chdir` to the EXE's directory** (`25979-26012`) — behaviour programs depend on.
11. `command_initialize(argc, argv)`; keyboard state (also instantiates the `GLUTEmu` singleton on the main thread, `26019-26021`); BIOS keyboard ring pointers at `cmem[0x41a..0x41d]`.
12. Load default palettes and 8x8 / 8x16 charsets from embedded byte arrays (`26031-26066`; arrays at `716`, `1248`, `2310`, `2345`).
13. `qbg_screen(0,...)` → SCREEN 0, 80x25 (`26068`). If `console`, create `console_image` (80x25 text image with `.console=1`).
14. Register devices 1 (keyboard) and 2 (mouse); `QB64_GAMEPAD_INIT()` if `DEPENDENCY_DEVICEINPUT`.
15. `libqb_http_init()`, `libqb_glut_presetup()` (creates window unless `$SCREENHIDE`).
16. Start QBMAIN thread, start TIMERTHREAD, set `lock_display_required=1`, then `libqb_start_main_thread()` which never returns.

`QBMAIN` (`qbx.cpp:1562`): `fpu_reinit()` (x87 control word 0x37F — 64-bit precision, round-to-nearest; `rounding.h:92`), installs SIGFPE → `error(11)` and SIGSEGV logger, creates the sub/function-scope `mem_lock`, then includes generated `maindata.txt`, `mainerr.txt`, `runline.txt`, event re-entry jump tables, `chain_input()`, and `main.txt`.

### 1.3 GUI main loop

`GLUTEmu::MainLoop` (`glut-emu.cpp:1560-1597`): `while(!glfwWindowShouldClose)` → `glfwPollEvents()` → `MessageProcess()` (drains a mutex-guarded queue of `Message` objects posted by other threads, woken with `glfwPostEmptyEvent`, `glut-emu.cpp:1608-1614`) → idle function.

Every `GLUTEmu_*` mutator checks "am I the main thread"; if not it posts a message (some wait on a `completion` for a reply). This is the mechanism that makes window calls from the QBMAIN thread legal.

`GLUT_IDLE_FUNC` (`libqb.cpp:24099`): error-accumulating frame limiter to `max_fps` (default 60, `libqb.cpp:94`; changed by `_FPS`, clamp 1..200, `12917`), then `GLUTEmu_WindowRefresh()` which leads to `GLUT_DISPLAY_REQUEST` (`25310`). On macOS the gamepad is polled here because it must be on the GLUT thread.

`$SCREENHIDE` programs: window creation is deferred; the main thread blocks on `glut_thread_starter` completion until `_SCREENSHOW` calls `libqb_start_glut_thread()` (`main-thread-gui.cpp:86-142`). `NEEDS_GLUT` / `OPTIONAL_GLUT` macros guard window APIs (`main-thread.h:26-39`).

### 1.4 `$CONSOLE:ONLY`

- Build defines `DEPENDENCY_CONSOLE_ONLY` → `QB64_GUI` undefined (`common.h:22-24`), no GL headers.
- Links `main-thread-console.cpp` + `window-console.cpp` instead of the `-gui` variants (`libqb\build.mk:29-30, 54-55`). All window functions become stubs.
- `libqb_start_main_thread()` just calls `MAIN_LOOP(NULL)` on the initial thread; no window, no GL thread. `display()` still runs but returns early because `screen_hide`.
- `new_hardware_img` is a stub (`libqb.cpp:196-231`).
- PRINT goes to `std::cout` when `write_page->console` (`libqb.cpp:10586-10598`); console input via `func__getconsoleinput` (`libqb\src\console.cpp:143`, Windows console API; mostly Windows-only).
- Note: `parts\audio`, `data`, `gui`, `font`, `image` are *always* compiled with `-DDEPENDENCY_CONSOLE_ONLY` just to keep GL headers out (`parts\audio\build.mk:16-19`) — a hack, not a semantic statement.

### 1.5 Locking (all ad hoc)

| Mechanism | Where | Protocol |
|---|---|---|
| `lock_display` 0/1/2 | `libqb.cpp:5828`, requests at `6938`, `7621`, `18768`; ack in `display()` `26312`, `27034` | QBMAIN sets 1 and spins `Sleep(0)` until `display()` on MAIN_LOOP sets 2; QBMAIN changes screen mode/font; sets 0. Plain non-atomic `int32`. |
| `lock_mainloop` 0/1/2 | `libqb.cpp:573`, `26136-26140` | Same handshake to park MAIN_LOOP entirely. |
| `display_lock_request/confirmed/released` (int64 counters) | `libqb.cpp:436-438`, render side `25423-25440`, BASIC side `qbx.cpp:1514-1520` | Render thread requests exclusive time to run `SUB _GL`; QBMAIN acknowledges at its next `evnt()` and **blocks until SUB _GL returns**. `gui_modal_lock_begin/end` (`448-478`) patch the deadlock when QBMAIN is stuck in a native dialog (the only `std::atomic` here). |
| `ontimerthread_lock` 0/1/2 | `qbx.cpp:1242-1268, 1366` | Spin handshake so `ontimer` can be `realloc`'d. `stop_timers()` busy-waits with no sleep. |
| `autodisplay` 1/0/-1 | `libqb.cpp:579`, `20344-20355`, `27038` | `_DISPLAY` sets -1 and waits for `display()` to flip it to 0. |
| `display_frame[3].state` | `libqb.cpp:100-114` | Lock-free triple buffer between `display()` (EMPTY→BUILDING→READY) and `GLUT_DISPLAY_REQUEST` (READY→DISPLAYING→EMPTY). Non-atomic. |
| `list_new_threadsafe` | `qblist.cpp:23` | Mutexes on add/remove only; used for `hardware_img_handles`. |
| `RingBuffer<T,N>` | `ring-buffer.h` | SPSC atomic ring: `_KEYHIT` (8192), INP(&H60) (4096), mouse events (65536). `OverwriteOnFull=true` variant is documented as *not* safe for general concurrency. |
| `lprint_locked`, `shell_call_in_progress`, `suspend_program`, `stop_program`, `close_program`, `exit_ok` | globals | Polled flags. |

There is essentially **no memory-model-correct synchronisation** between QBMAIN and the two other first-party threads; correctness relies on x86 TSO and polling.

### 1.6 `evnt()` — the cooperative yield point

The compiler emits `if (qbevent) { evnt(line); if (r) goto retry; }` style checks between statements. `evnt()` (`qbx.cpp:1508-1541`):
1. clears `qbevent`;
2. acknowledges a pending display lock and waits for `SUB _GL` to finish;
3. while `suspend_program || stop_program`: `end()` if stopping;
4. if an error is pending: `error_set_line`, `fix_error()`, set `r=1` if RESUME-retry;
5. else `events()` (ON STRIG, ON KEY, ON TIMER dispatch, `qbx.cpp:1401-1500`).

`_LIMIT`, `_DELAY` and `SLEEP` call `evnt(0)` every ~9 ms while waiting (`datetime.cpp:96, 149`; `libqb.cpp:16264`).

### 1.7 Shutdown

- Window close button → `GLUT_EXIT_FUNC` (`libqb.cpp:27116`) cancels the GLFW close and sets `exit_value |= 1`. Ctrl+Break sets bit 2.
- `MAIN_LOOP` sees `exit_value` and, unless `exit_blocked` (program has called `_EXIT`, `libqb.cpp:21895`), jumps to `end_program` (`26255`): `stop_program=1; qbevent=1;` wait until `exit_ok==3` (bit 1 set by QBMAIN in `end()` `5906-5913`, bit 2 by TIMERTHREAD `qbx.cpp:1393-1396`); flush LPRINT; `sub_close` all; `libqb_http_stop()`; gamepad shutdown; `libqb_exit(exit_code)`.
- `libqb_exit` (`main-thread-gui.cpp:148-155`) marshals `exit()` to the GLFW thread via `GLUTEmu_ProgramExit` because of atexit ordering.
- BASIC `END` → `sub_end` (`libqb.cpp:19281`): closes files, prints **"Press any key to continue"** on the last row and waits for a key (unless hidden / console), then `close_program=1; end();`. `SYSTEM` skips the prompt (generated code). `end()` never returns: it parks the QBMAIN thread forever in `Sleep(16)`.
- QBMAIN is never joined; the process dies via `exit()`.

---

## 2. Core data model

### 2.1 `qbs` strings

Struct: `libqb\include\qbs.h:14-31`. Fields: `chr`, `len` (signed int32), `in_cmem`, `cmem_descriptor`(+offset), `listi`, `tmp`, `tmplisti`, `fixed`, `readonly`, `field`.

Storage (`libqb\src\qbs.cpp`):

| Pool | Definition | Notes |
|---|---|---|
| Descriptors | `qbs_malloc` slabs of 65,536 `qbs` + free list (`qbs.cpp:19-62`) | Slabs never freed. |
| String heap | `qbs_data` (starts 1 MB), bump pointer `qbs_sp` (`qbs.cpp:78-80`) | Each `qbs_new` reserves `len+32` bytes of slack. |
| Live list | `qbs_list[]` of descriptor pointers in allocation order (`qbs.cpp:65-67`) | -1 = hole. |
| Temp list | `qbs_tmp_list[]`, `qbs_tmp_list_nexti` (`qbs.cpp:72-75`) | Stack discipline. |
| cmem strings | `qbs_cmem_list[]`, data inside DBLOCK (`qbs_cmem.cpp:16-20`) | 64 KB hard limit → fatal errors 513-515. |

Key operations:

- `qbs_new(size,tmp)` (`qbs.cpp:245`): bump allocate; if full → `qbs_concat()` (`152`) which **compacts by sliding every live string down** (order-preserving, fixing `chr`) and doubles the heap with `realloc`, rebasing every `chr`. Therefore **any `qbs` allocation can move every other non-fixed string's bytes**; raw `chr` pointers must not be held across allocations.
- `qbs_new_txt` / `qbs_new_txt_len` (`191`, `210`): temp + `readonly` descriptor pointing at a C literal (no copy).
- `qbs_new_fixed(offset,size,tmp)` (`226`): descriptor over foreign memory (fixed-length strings, UDT members); if non-temp and inside DBLOCK also gets a cmem descriptor.
- `qbs_new_cmem` (`269`) → `qbs_create_cmem` (`qbs_cmem.cpp:92`): string data in DBLOCK growing up from `qbs_cmem_sp` (starts 256), 4-byte `[len][offset]` QB-style descriptor allocated downward from `cmem_sp` (starts 65536) — this is what `VARPTR`/`SADD`-style code sees.
- `qbs_set(dest,src)` (`289-403`): the assignment operator.
  - fixed dest: copy and space-pad / truncate;
  - if src is a real temp (not fixed/readonly, same cmem-ness): **steal** src's buffer and list slot, free src descriptor;
  - src fits in place: `memcpy`;
  - dest is last in heap or has room before next live string: grow in place;
  - else unlist/relist at heap top (compacting if needed);
  - always frees a temp src.
- `qbs_add` (`405`): returns the other operand unchanged if one is empty (so the result may alias an input and may or may not be a temp), else a new temp.
- `qbs_free` (`82`): handles FIELD unlink, temp-list hole, heap-top retreat.
- `qbs_cleanup(base, passvalue)` (`qbs.h:97-106`): pops and frees temps above `base`. Generated code brackets each statement with `qbs_tmp_base = qbs_tmp_list_nexti; ... qbs_cleanup(qbs_tmp_base, value)`.
- `qbs_maketmp` (`279`): used for function return values.

Functions that consume a temp argument free it themselves (or leave it for `qbs_cleanup`); conventions are per-function and undocumented.

### 2.2 Conventional memory (`cmem`)

- `uint8_t cmem[1114099]` (`qbx.cpp:326`) = full real-mode 1 MB + HMA.
- Layout comment `libqb.cpp:5860`: `[1280][DBLOCK][STATIC-> <-DYNAMIC][A000-]`.
  - 0..0x4FF: BIOS data area. Emulated cells: keyboard ring head/tail `0x41a/0x41c` with data at `0x41e..` (INKEY$ reads straight from it, `keyboard.cpp:1294-1318`), tick counter `0x46c-0x46e` (`libqb.cpp:26150-26158`).
  - 0x500 (seg &H50 = 80): **DBLOCK**, 64 KB "DGROUP": the default `DEF SEG`, home of cmem variables and strings. `dblock` = its address.
  - `cmem_static_pointer` from `1280+65536` upward: static far arrays.
  - `cmem_dynamic_base` from 0xA0000 downward: `cmem_dynamic_malloc/free` (`libqb.cpp:5992-6080`), a linked-list first-fit allocator, 16-byte aligned, max 64 KB per block, static table of 147,136 links (~5.9 MB BSS).
  - 0xA0000: **SCREEN 13 page 0 lives here** (`imgframe(&cmem[655360],320,200,13)`, `libqb.cpp:7013`).
  - 0xB8000: **SCREEN 0 pages live here** when they fit (`libqb.cpp:7328`), so `DEF SEG=&HB800: POKE` works.
- Which variables go into cmem is decided by the compiler (`ISINCONVENTIONALMEMORY` flag, `common.h:104`), generally when the program uses VARPTR/VARSEG/DEF SEG/CALL ABSOLUTE etc.
- `sub_defseg` / `func_peek` / `sub_poke` (`libqb.cpp:6084-6115`): range-checked to QB's `-65536..65535`, error 6 otherwise.
- `varptr_dblock_check` / `varseg_dblock_check` (`qbx.cpp:482-498`): VARPTR/VARSEG for cmem objects.
- `mem_static_malloc` / `mem_static_restore` (`libqb.cpp:5942-5963`): separate non-cmem bump arena for SUB/FUNCTION locals; grows by abandoning the old block (deliberate leak, comment at `5920-5936`).
- `sub_bsave` / `sub_bload` (`16735`, `16781`) operate on `defseg`.

### 2.3 Arrays

No C struct — an array is a `ptrszint[]` descriptor laid out by generated code:

| Slot | Meaning | Evidence |
|---|---|---|
| `[0]` | data pointer (or `nothingvalue` when erased) | `array-copy.cpp:21-25` |
| `[2]` | flags; bit 0 = allocated/DIMmed | `libqb.cpp:16292` |
| `[4*k]`, `[4*k+1]` | lower bound and element count of a dimension, dimensions stored **in reverse order** (`k = num_indexes - index + 1`) | `libqb.cpp:16291-16306` |
| `[4*k+2]` | per-dimension multiplier (inferred; not verified) | — |

Remaining slots (1, 3, lock id/offset for `_MEM`) were not determined from the runtime side; they are defined by the code generator. Bounds check helper: `array_check` (`qbx.cpp:474`) → error 9. Bit-packed `_BIT` arrays use `getbits/setbits` (`bitops.h:7-31`). `_ARRAYCOPY` runtime: `libqb\src\array-copy.cpp` (954 lines).

### 2.4 `_MEM`

`mem_block` (`memblock.h:5-14`): `offset, size, lock_id, lock_offset, type, elementsize, image, sound`. `mem_lock` (`memblock.h:25-36`): `id`, `type` (0 none, 1 malloc, 2 image, 3 sub/func scope, 4 array, 5 sound), `offset`. Validity check = `((mem_lock*)blk.lock_offset)->id == blk.lock_id`; freeing sets `id=0`. Locks come from 10,000-entry slabs with a free list (`memblock.cpp:11-44`); ids start at 1073741823. `func__memnew` `memblock.cpp:90`, `sub__memfree` `46`, `sub__memcopy` `270`, `func__memimage` `libqb.cpp:24034`, `func__memsound` in audio. Errors 300-313. The `mem_block` struct layout is ABI with generated code and with BASIC `_MEM` UDT.

### 2.5 Images and screen pages

- `img_struct` (`graphics.h:17-59`); global growable array `img[]` (4096 at a time, `libqb.cpp:2403-2410`) with free list `fimg[]`. `newimg` `2620`, `freeimg` `2645`, `imgframe` (wrap external buffer) `2793`, `imgnew` `2920`, `imgrevert` `2666`.
- **Handles**: BASIC sees the *negated index* (`-i`). 0 = current screen; positive values are legacy page numbers resolved by `page[]`/`validatepage` (`6813`). Valid software handles are < -1 (indexes 0,1 reserved). Hardware images: `list` index + `HARDWARE_IMG_HANDLE_OFFSET` (-16777216); software floor `SOFTWARE_IMG_HANDLE_MIN` -8388608 (`186-187`) — chosen so handles survive storage in a SINGLE.
- `compatible_mode`: 0,1,2,7,8,9,10,11,12,13,256,32. `bytes_per_pixel` 1 (indexed), 2 (text cell: char+attr), 4 (BGRA).
- Pixel format is **BGRA little-endian = 0xAARRGGBB**; palettes are `uint32[256]` in the same format.
- `pages`/`page[]`: page number → img index. Non-zero pages are created lazily, share page 0's palette pointer and font, and get `IMG_SCREEN` (`6813-6842`).
- `write_page`, `read_page`, `display_page` pointers + `*_index` (`2468-2474`) = `_DEST`, `_SOURCE`, visual page. They are re-pointed after `img` realloc (`2634-2636`).
- "apm" block in `img_struct` (`graphics.h:45-58`): VIEW/WINDOW/DRAW state migrated between pages on active-page change.
- 32-bit blending uses lazily built LUTs: `cblend` 16 MB (alpha×src×dst) and `ablend` 64 KB (`init_blend` `2418-2466`), only allocated when the first 32-bit image is created.

### 2.6 Palettes

`palette_256` / `palette_64` from embedded data (`libqb.cpp:2310`, `2345`, loaded `26032-26039`); `restorepalette` per mode (`2477-2566`); SCREEN 10 special `pal_mode10` (`26042-26060`). `PALETTE` = `qbg_palette` `6405`; `PALETTE USING` `23885`; `_PALETTECOLOR` `18169/18201`; `_COPYPALETTE` `18233`; DAC ports 3C7/3C8/3C9 (§2.11).

### 2.7 Fonts

Parallel global arrays `font[]`, `fontwidth[]`, `fontheight[]`, `fontflags[]` (`libqb.cpp:684-688`, grown as needed, `lastfont`). Indexes 8, 14, 16 = built-in VGA fonts (9, 15, 17 = double-width); index ≥ 32 → `font[i]` is a FreeType handle from `FontLoad` (`parts\video\font\font.cpp:631`). `func__loadfont` `18604`, `sub__font` `18714`, `selectfont` `4653`, `sub__freefont` `18884`. Built-in glyphs: `charset8x8[256][8][8]`, `charset8x16[256][16][8]` (one byte per pixel; `3026-3027`); the 8x14 font is not a separate table here (derivation not traced).

### 2.8 Sound handles

Entirely inside `AudioEngine` (`parts\audio\audio.cpp:28`): `std::vector<SoundHandle*>` (`2148`); handle 0 reserved for PLAY/SOUND voice 0; handle reuse; `mem_lock` integration for `_MEMSOUND`.

### 2.9 File handles

Two-level: BASIC file number → `gfs_fileno[]` → GFS index → `gfs_file_struct` (`gfs.h:39-88`). Negative numbers = "special handles" (`special_handle_struct` `libqb.cpp:121-140`) for TCP streams/hosts/HTTP, sharing GET/PUT/EOF/LOF/CLOSE. GFS has two backends: `GFS_WINDOWS` (Win32 `HANDLE`, supports LOCK and COM ports) and `GFS_C` (`std::fstream`, no locking) (`gfs.h:8-16, 56-63`).

### 2.10 Error state and ON ERROR

Globals (`error_handle.cpp:22-32`): `new_error` (pending), `error_err` (ERR), `error_erl`, `error_occurred`, `error_goto_line` (ON ERROR target id, 0 = none), `error_handling` (inside handler), `error_retry`, `error_handler_history`.

Flow:
1. Runtime function calls `error(n)` (`error_handle.cpp:435`): sets `new_error=n` (first error wins) and `qbevent=1`. **It returns to the caller** — no exception/longjmp.
2. Consequently almost every runtime function starts with `if (is_error_pending()) return;` (111 occurrences in `libqb.cpp`) so the rest of the statement becomes a no-op.
3. Generated code reaches `evnt()` → `fix_error()` (`364-433`):
   - No handler, already handling, or error 300-315: dialog "Unhandled Error #n / Line: X (in main module|file) / Continue?" via `gui_alert` (tinyfiledialogs). "No" → `close_program=1; end()`. Env `QB64PE_NOPROMPT=y|continue` routes to stderr instead (`295-360`).
   - Handler present: set `error_err`, `error_erl=last_line`, `error_occurred=1`, then **call `QBMAIN(NULL)` recursively** (`431`, with a "FIXME: EWWWWW" comment). The re-entered QBMAIN runs `mainerr.txt` which dispatches on `error_occurred`/`error_goto_line` to the handler label; RESUME / RESUME NEXT jump back via generated tables. Native stack grows per handled error and is never unwound.
4. Critical (never trappable) errors: 257 and 502-518 (out of memory), **11 division by zero**, 256 stack, 259-261 DLL, 270/271 GL scope (`error_handle.cpp:443-497`). Note that QB45's error 11 is trappable; here integer divide by zero is fatal (delivered via SIGFPE handler `qbx.cpp:1545`). Float division by zero follows IEEE (inf) — behaviour difference from QB45 worth a design decision.
5. `libqb_check_stack()` (`91-142`) emitted per user SUB/FUNCTION → error 256 with 256 KB reserve.
6. `$ErrorLocation:ON` line tracking: `error_track_line` (`269`).
7. `ERR`/`ERL`/`_ERRORLINE`/`_INCLERRORLINE`/`_INCLERRORFILE$`/`_ERRORMESSAGE$` accessors `233-280`. Message table `144-227`.

GOSUB/RETURN: `return_point[]` stack of label ids (`qbx.cpp:334-336`, `more_return_points` `libqb.cpp:6124`); RETURN is a generated `switch`.

### 2.11 Events

| Event | State | Trigger source | Dispatch |
|---|---|---|---|
| ON TIMER | `ontimer[]` (`common.h:108-116`), slots via `_FREETIMER` (`qbx.cpp:1254`) | TIMERTHREAD | `events()` `qbx.cpp:1474-1499`; generated `ontimer.txt` switch on `id`; `state` 0/1/2 prevents re-entry; sets `sleep_break` |
| ON KEY(n) | `onkey[1..31]` (`key-events.h:6-21`) | keyboard callbacks in `keyboard.cpp` (state is a counter → queued presses) | `events()` `1444-1471` |
| ON STRIG | `onstrig[65536]` = 256 controllers × 256 buttons (`qbx.cpp:1133`) | gamepad callbacks | `events()` `1406-1441` |
| ON ERROR | §2.10 | | |
| KEY ON/LIST/n,string | `key_update` `libqb.cpp:23551`, `key_on/off/list/assign` `23796-23878` | | function-key bar on last row |

ON COM, ON PEN, ON PLAY were not found in the runtime (compiler-side handling not checked). ON...GOSUB handlers run as nested generated-code calls while `timer_event_occurred` etc. counters (`qbx.cpp:302-309`) steer re-entry into `QBMAIN`.

### 2.12 Hardware emulation

| Feature | Location | What is emulated |
|---|---|---|
| `OUT` | `libqb.cpp:12807-12859` | &H3C0 (blink enable bit 3), &H3C7/&H3C8 DAC index, &H3C9 DAC data (6-bit → 8-bit with `qbr(data*4.063492-0.4999999)`). Everything else silently ignored (`unsupported_port_accessed=1`). |
| `INP` | `16309-16375` | &H3C9 palette readback (÷3.984376), &H3DA bit 3 vertical retrace (pulsed by MAIN_LOOP, `26171-26173`, `26209`), &H60 keyboard scancodes (set-1, release = +128) from a ring. |
| `WAIT` | `16377-16419` | Polls `func_inp`; returns immediately for unsupported ports. |
| `PEEK/POKE/DEF SEG` | `6084-6115` | Raw `cmem`. No write hooks: video memory works only because pages alias cmem. |
| `CALL ABSOLUTE` | `call_absolute` `17350-17367` + `cpu_call` `5510-5830` | Tiny real-mode x86 interpreter: prefixes 66/67/seg, MOV forms (88-8E, A0-A3, B0-BF, C6, C7), PUSH/POP (50-5F, 68, 6A, segment regs, 8F, FF /6), RETF (CA, CB), INT (CD), 0F A0/A1/A8/A9. Anything else → "Unknown Opcode (xx)" alert. Enough for the classic mouse-interrupt stub only. ModRM decoding `rm8/rm16/rm32` `4931-5510`, `sib` `4871`. |
| `CALL INTERRUPT[X]` | `14792`, `14853`, `call_int` `17369-17428` | Only INT 33h mouse: AX=0 reset, 1 show, 2 hide, 3 status (with QB coordinate doubling for 320-wide modes and ×8 text coordinates), 7/8 no-op. Flags ignored. |
| `VARPTR$` | `func_varptr_helper` `6137` | 3-byte type+offset string, consumed by DRAW/PLAY "X" (`draw` reads `cmem[1280+offset]`, `20401-20441`). |

---

## 3. Graphics

### 3.1 SCREEN modes (`qbg_screen`, `libqb.cpp:6845-7450`)

| Mode | Size | bpp | Default font | Backing |
|---|---|---|---|---|
| 0 | 80x25 default (WIDTH: 40/80 × 25/43/50 etc.) | text cells (2 bytes) | 16 | `cmem[0xB8000]` when it fits (`7328`), else heap |
| 1 | 320x200 | 2 | 8 | heap (`7203`) |
| 2 | 640x200 | 1 | 8 (rendered stretched) | `7184` |
| 7 | 320x200 | 4 | 8 | `7146` |
| 8 | 640x200 | 4 | 8 | `7128` |
| 9 | 640x350 | 4 (16 of 64) | 14 | `7108` |
| 10 | 640x350 | 2 (4 of 9) | 14 | `7085` |
| 11 | 640x480 | 1 | 16 | `7049` |
| 12 | 640x480 | 4 | 16 | `7029` |
| 13 | 320x200 | 8 | 8 | **`cmem[0xA0000]`** (`7013`) |
| `_NEWIMAGE(w,h,bpp)` | any | 0 (text), 1,2,7-13, 256, 32 | — | heap; `SCREEN handle` passes negative mode (`6868-6879`) |

Modes 3-6 and >13 → error 5 (`6881-6890`). All indexed modes are stored **one byte per pixel** (no planar emulation). A mode change takes `lock_display`, frees old pages in reverse, migrates state; `sub_screen_keep_page0` covers `SCREEN _NEWIMAGE` hand-over. `width8050switch` (`2386`) allows auto-switch to 50 rows on LOCATE beyond 25. `WIDTH` = `qbsub_width` (`7496-8243`, ~750 lines, includes console and LPRINT width paths). `PCOPY` `7451`.

### 3.2 Text mode rendering

- Text pages are arrays of `(char, attr)` bytes, cleared to `0x0720`.
- `printchr` (`10128-10350`) writes cells in text mode, or blits glyphs (built-in charset or FreeType-rendered) in graphics modes, honouring `_PRINTMODE` (keep/only/fill background).
- `qbs_print` (`10579-10967`): control characters (7 beep, 9 tab, 10/13, 11 home, 12 cls, 28-31 cursor moves) unless `_CONTROLCHR OFF`; "holding cursor" state (`holding_cursor`) reproduces QB's deferred wrap at column 80; scrolling within VIEW PRINT via `newline` (`10370`); comma zones via `tab()` (`10451`), `func_tab` `16421`, `func_spc` `16523`.
- Screen composition of text pages happens in `display()` (`26386` onward): per cell diff against `screen_last`, cursor shape from `cursor_firstvalue/lastvalue`, blink of attribute bit 7 controlled by `H3C0_blink_enable`, blink phase from `frame & 8` (~32 fps counter). Output is a BGRA frame.
- `func_screen` (SCREEN(row,col[,colorflag])) `16620`; `func_csrlin` `16156`; `func_pos` `16175`; `LOCATE` `11480`; `CLS` `11277` (method 0/1/2 semantics); `COLOR` `6532`; `VIEW PRINT` `11095`.

### 3.3 Software drawing primitives (all operate on `write_page`)

| Statement | Function | Lines | Notes |
|---|---|---|---|
| PSET/PRESET | `sub_pset` / `sub_preset`, raw `pset`, `pset_and_clip` | 10058, 10111, 2568, 8243 | 32-bit path alpha-blends via LUTs, special cases for alpha 0/127/128/255 |
| POINT | `func_point`, `point` | 9992, 9983 | also POINT(n) cursor queries |
| LINE | `sub_line` → `qb32_line` (styled), `fast_line`, `qb32_boxfill`, `fast_boxfill`; `lineclip` | 8701, 8613, 8545, 8290, 8443, 6185 | float coords through WINDOW/VIEW transform, `qbr_float_to_long` rounding; 16-bit style mask |
| CIRCLE | `sub_circle` | 9727-9982 | arcs, negative-angle radius lines, aspect; default aspect depends on mode |
| PAINT | `sub_paint` (solid), `sub_paint` (tile string overload), `sub_paint32`, `sub_paint32x` | 9263, 9528, 8777, 9074 | scanline flood using global work buffers; tile patterns decoded by `getptcol_{1,2,4,8}bpp` (9475-9527) |
| DRAW | `sub_draw`, `draw_num` | 20513-21036, 20365 | full macro language incl. TA, S, P, X + VARPTR$, "=" variable refs into cmem |
| GET/PUT (graphics) | `sub_graphics_get`, `sub_graphics_put` | 15419, 15679-16155 | QB-compatible packed array format per mode (bit-planar layout for EGA/VGA 16-colour modes); PSET/PRESET/AND/OR/XOR; QB64 `_CLIP` + mask |
| VIEW / WINDOW / PMAP | `qbg_sub_view`, `qbg_sub_window`, `func_pmap` | 11134, 10968, 16595 | |
| PALETTE | `qbg_palette` | 6405 | |

Coordinates: BASIC passes floats; conversion uses the x87 banker's rounding helpers — pixel-exact output depends on it.

### 3.4 `_PUTIMAGE` (`sub__putimage`, `libqb.cpp:3029-4652`)

1,600 lines, one function, ~30 `goto` labels. Decodes the `passed` bitmask, resolves STEP/partial rectangles, clips, then jumps to a specialised inner loop by (src bpp, dst bpp, clear-colour, alpha, stretch, mirror/flip): `put_32`, `put_32_noalpha`, `put_8`, `put_8_clear`, `put_8_32`, `put_8_32_clear`, each ×{plain, `_stretch`, `_mirror`} (`3918-4650`). `_SMOOTH` is ignored in software. If either end is a hardware image it instead **queues a `HARDWARE_GRAPHICS_COMMAND__PUTIMAGE`** (`3060-3105`). Source==dest makes a temporary copy (`3168-3175`). 32→8 bpp is an error.

Related: `_COPYIMAGE` `17482` (mode 33 → hardware image), `_FREEIMAGE` `17555`, `_SOURCE/_DEST` `17634-17682`, `_BLEND/_DONTBLEND` `17690/17723`, `_CLEARCOLOR` `17754`, `_SETALPHA` `17852`, `_WIDTH/_HEIGHT/_PIXELSIZE` `17955-18062`, `_RGB/_RGBA/_RED...` `19017-19280` (palette matching via `matchcol` `18971`), `_PRINTSTRING` `18285`, `_PRINTWIDTH` `18559`, HSB helpers in `libqb\src\graphics.cpp:29-183`.

### 3.5 `_MAPTRIANGLE`

`sub__maptriangle` in `libqb\src\graphics.cpp:252` to end (~2,660 lines): software affine textured-triangle rasteriser with many specialised inner loops; for hardware images queues `MAPTRIANGLE` / `MAPTRIANGLE3D` commands. `_DEPTHBUFFER` `graphics.cpp:184`.

### 3.6 Hardware images and the command queue

- `hardware_img_struct` (`graphics.h:137-153`): GL texture (created lazily on the GL thread from `software_pixel_buffer`, `libqb.cpp:371`), optional FBO (`dest_context_handle`) and depth texture (`409`), NPOT fallback (`NPO2_texture_generate` `238`, then mipmap fallback).
- QBMAIN never touches GL. It appends `hardware_graphics_command_struct` nodes (`graphics.h:155-198`) to a linked list tagged with `order = display_frame_order_next` — i.e. bound to the software frame they belong to.
- `GLUT_DISPLAY_REQUEST` replays commands for the frame being shown; older ones are executed only if they target persistent hardware images, then marked `remove`. `flush_old_hardware_commands` (`2964`) reclaims nodes from the QBMAIN side and converts `FREEIMAGE_REQUEST` into `FREEIMAGE` so textures are deleted on the GL thread.
- GL helpers: `prepare_environment_2d` `24284`, `set_view` `24601`, `set_render_source/dest` `24761/24791`, `set_smooth/alpha/depthbuffer/cull_mode/texture_wrap` `24481-24600`, `hardware_img_put` `24839`, `hardware_img_tri2d` `24973`, `hardware_img_tri3d` `25203`, `clear_depthbuffer` `25192`, `hardware_buffer_flush` `24457`. All **fixed-function OpenGL 1.x/2.x compatibility profile** with `EXT_framebuffer_object` and `GLU` (`gluBuild2DMipmaps`).

### 3.7 How a frame reaches the window

1. **MAIN_LOOP thread**, every ~32 ms if `autodisplay`, or QBMAIN via `_DISPLAY`: `display()` (`libqb.cpp:26297-27042`). Picks an EMPTY (or oldest READY) slot of `display_frame[3]`, marks BUILDING, converts `display_page` to 32-bit BGRA: text renderer; indexed → palette lookup loop (`27001-27012`); 32-bit → `memcpy`. Skips rebuild if pixels and palette are unchanged vs `pixeldata`/`paldata` cache (`screen_last_valid`). Marks READY and records `last_hardware_display_frame_order`.
2. **GL thread**, up to `max_fps`: `GLUT_DISPLAY_REQUEST` (`25310-25713`) picks the newest READY frame (→ DISPLAYING, frees older ones). If nothing changed and no GL layer is needed it returns without swapping. Otherwise:
   - `window_update_for_frame` (`window-gui.cpp:82`) resizes the window to the frame size / applies pending fullscreen changes;
   - `prepare_environment_2d` computes scale/letterbox (`environment_2d__*`, `libqb.cpp:81-92`);
   - loop `level = 0..5`: clear, then by `_DISPLAYORDER` (`24154`; defaults: 1 `_SCREEN` software frame as a textured quad, 2 `_HARDWARE`, 3 `_GLRENDER` = user `SUB _GL`, 4 `_HARDWARE1`), level 5 letterbox bars;
   - `GLUTEmu_WindowSwapBuffers()` with `glfwSwapInterval(1)` (vsync, `glut-emu.cpp:511`).
3. `SUB _GL` runs **on the GL thread** while QBMAIN is parked in `evnt()` (§1.5). The `_gl*` BASIC commands map to GL calls through generated wrappers in `parts\core\gl_header_for_parsing\temp\gl_helper_code.h` (included by `qbx.cpp:56`, produced by `gl_kit.bas` from a reference `gl.h`). Calling them outside `SUB _GL` → error 270; END inside → 271.

`_AUTODISPLAY` `20340`; `_DISPLAY` `20344`; `display_now` `21906`; `_LIMIT` is a pure sleep-based limiter on QBMAIN (`datetime.cpp:105-164`), not vsync-coupled; `_FPS` controls the GL thread's cap.

### 3.8 Resize, scaling, fullscreen

`libqb\src\window-gui.cpp` (423 lines): `$RESIZE:ON|STRETCH|SMOOTH` (`ScreenResize`, `ScreenResizeScale` set in `dyninfo.txt`), `_RESIZE`, `_RESIZEWIDTH/HEIGHT`, snap-back to frame size when resizing is off (`resize_snapback`), aspect-ratio constraint in stretch mode (`43-66`), `_FULLSCREEN` `_STRETCH|_SQUAREPIXELS|_OFF [,_SMOOTH]` implemented as GLFW borderless/monitor fullscreen with GL scaling + letterbox (no display mode switch), `_ALLOWFULLSCREEN` (Alt+Enter policy), `_SCREENMOVE`, `_SCREENX/Y`, `_DESKTOPWIDTH/HEIGHT`, `_TITLE`, `_ICON` (`libqb.cpp:20317`), `_SCREENICON`, `_SCREENHIDE/SHOW`, drag-and-drop (`_ACCEPTFILEDROP`, `_DROPPEDFILE$`), `_WINDOWHANDLE`, `_WINDOWSIZELIMIT`. `_SCALEDWIDTH/HEIGHT` `libqb.cpp:25763`. HiDPI hints set at window creation (`main-thread-gui.cpp:31-39`): 4x MSAA, scale-to-monitor.

---

## 4. Input

### 4.1 Keyboard (`libqb\src\keyboard.cpp`, 2,510 lines; `keyboard.h` 323 lines of key constants)

GLFW key callback `GLUT_KEYBOARD_BUTTON_FUNC` (`2399`) and char callback `GLUT_KEYBOARD_CHARACTER_FUNC` (`2119`) run on the GL thread and funnel into `keyboard_keydown` (`1501-1938`) / `keyboard_keyup` (`1458`), which fan out to **five consumers**:

| Consumer | Storage | API |
|---|---|---|
| INKEY$ / INPUT | Emulated BIOS ring in `cmem[0x41e..]`, head `0x41a`, tail `0x41c`, 16 two-byte entries (`keyboard_push_bios_keystroke` `1122`) | `qbs_inkey` `1294`: 1-byte string or 2-byte `CHR$(0)+scancode` |
| `_KEYHIT` | `RingBuffer<int64,8192>` (`665`) | `func__keyhit` `1274`: positive = press, negative = release; codes are ASCII/CP437, `256*scancode` for extended keys, 100000+ range for QB64 virtual keys, Unicode ≥ 0x40000000-style values |
| `_KEYDOWN` | held-key list (`keyheld` `831`, add/remove `1351/1360`) | `func__keydown` `1282` |
| `INP(&H60)` | `RingBuffer<uint8,4096>` of set-1 scancodes (`scancodedown/up` `1370/1381`; GLFW→DOS scancode table `2176`) | `func_inp` |
| `_DEVICES` device 1 | `devices[1]` events, 512 buttons | `Keyboard_ReportDeviceEvent` `816` |

Plus ON KEY triggering and `PEEK(&H417)`-style shift flags (`keyboard_update_shift_state` `1939`). Large static tables map scancode × {normal, shift, ctrl, alt} to QB codes (`keyboard_scancode_lookup_table` `31`). CP437 ↔ Unicode via `_MAPUNICODE` (`libqb.cpp:23490`) and `codepage437_to_unicode16` (`499`). Layout-dependent characters are deferred to the char callback (`ShouldDeferToCharCallback` `999`, AltGr handling `885`). `_KEYCLEAR` `1320`; `_CAPSLOCK/_NUMLOCK/_SCROLLLOCK` `2047-2118`. A dead `QBVK_2_scancode` table remains in `MAIN_LOOP` (`libqb.cpp:26216-26246`).

`INPUT`/`LINE INPUT` keyboard editing is `qbs_input` (`libqb.cpp:11745-12789`, ~1,050 lines): snapshots the destination image, runs its own edit loop with cursor, re-validates numeric fields per variable type table (`qbs_input_variabletypes[]`), and redraws. (Body sampled only; the classic "Redo from start" string was not found by grep, so the retry UX may differ from QB45 — verify.) File/DATA numeric parsing: `n_inputnumberfromdata` `13885`, `n_inputnumberfromfile` `14132`, with `n_float/n_int64/n_uint64` converters `13551-13884`.

### 4.2 Mouse (`libqb\src\mouse.cpp`, 693 lines)

GLFW callbacks push `mouse_event` records into `RingBuffer<mouse_event,65536,overwrite>` (`36-49`). `_MOUSEINPUT` pops one into `current_gui_state`; `_MOUSEX/_MOUSEY` convert window pixels to image coordinates using the 2D environment scale/offset, and to character cells in text mode (`268-349`); `_MOUSEBUTTON(n)`, `_MOUSEWHEEL`, `_MOUSEMOVEMENTX/Y` (relative mode), `_MOUSEMOVE` (warp with suppression of the synthetic motion event), `_MOUSESHOW "style"` / `_MOUSEHIDE`, custom cursor from image. Also feeds `_DEVICES` device 2 and INT 33h (`mouse_get_int33_status` `394`, reads latest pushed state without needing `_MOUSEINPUT`). Console mouse: `console.cpp:246-262` (Windows).

### 4.3 Game controllers

- Glue: `parts\input\game_controller\game_controller.cpp` (241 lines) over vendored **libstem_gamepad** (Alex Diener; per-OS: WinMM `Gamepad_windows_mm.c`, Linux evdev, macOS IOKit). Gated by `DEPENDENCY_DEVICEINPUT`.
- Device model in `qbx.cpp:896-1131`: `devices[1..]` (`device_struct`, `game_controller.h:16-45`); each event is a packed record `[axes: double][wheels: double][buttons: uint8][int64 global order]`; queue grows to `QUEUED_EVENTS_LIMIT` 1024 then drops oldest. `_DEVICES`, `_DEVICE$`, `_DEVICEINPUT`, `_BUTTON`, `_BUTTONCHANGE`, `_AXIS`, `_WHEEL`, `_LASTBUTTON/_LASTAXIS/_LASTWHEEL`.
- Legacy `STICK` / `STRIG`: `func_stick` `libqb.cpp:23930`, `func_strig` `23977` map onto controllers.
- Poll site: `MAIN_LOOP` (Win/Linux) or GL idle (macOS).

### 4.4 Clipboard

`parts\os\clipboard\clipboard.cpp` (389 lines) over vendored **clip** (dacap; version not stamped in tree): `_CLIPBOARD$` get/set, `_CLIPBOARDIMAGE` get/set. Linux needs xcb + libpng.

---

## 5. Audio

Backend: **miniaudio 0.11.25** (`parts\audio\miniaudio\miniaudio.h:3748-3750`), high-level `ma_engine` + `ma_resource_manager` with a custom VFS for in-memory loads (`audio.cpp:131`, `2511-2560`). Engine initialises lazily at device default rate; decoders resample to it. The whole first-party engine is one class in `parts\audio\audio.cpp` (3,913 lines) with free-function wrappers at `3794-3913`. `stub_audio.cpp` exists for builds without `DEP_AUDIO_MINIAUDIO`.

| BASIC | Implementation |
|---|---|
| `SOUND`, `PLAY`, `BEEP`, `_WAVE` | `AudioEngine::PSG` (`audio.cpp:674-2147`): software tone generator with multiple voices (QB64pe extension: up to 4 PLAY strings / voices, waveforms square/saw/triangle/sine/noise/custom, volume, pan, ramps). MML parser constants at `692-714` (tempo 32-255, octave 0-7, lengths 1-64, default pause 1/8). Samples are pushed into a `RawStream`. `SOUND` duration is in 18.2 Hz ticks. `PLAY "MB"/"MF"` background/foreground. `sub_beep` lives in `parts\gui\gui.cpp:377`. `func_play` = PLAY(n) notes remaining. |
| `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWLEN`, `_SNDOPENRAW` | `RawStream` (`388-672`): mutex-guarded producer vector / consumer vector swapped by the audio callback, exposed as a custom `ma_data_source`. |
| `_SNDOPEN` etc. | `SoundHandle` (`2148`); miniaudio built-ins **WAV, FLAC, MP3**; **Ogg Vorbis** via stb_vorbis; custom decoder vtables (`3783-3791`, priority order): RAD v2 (`radv2`, Opal OPL3), HivelyTracker/AHX (`hvl_replay`), MIDI family, QOA, tracker modules (libxmp-lite: MOD/XM/S3M/IT). `_SNDCOPY`, `_SNDPLAYCOPY`, `_SNDPLAYFILE`, `_SNDVOL`, `_SNDBAL` (3D position), `_SNDLOOP`, `_SNDLIMIT`, `_SNDSETPOS/GETPOS/LEN`, `_SNDPAUSE/STOP`, `_SNDNEW`, `_MEMSOUND`. |
| MIDI | `midi_ma_vtable.cpp` + **libmidi** (SMF, RMI, MUS, XMI, HMI/HMP, GMF, LDS, MDS, RCP) + **foo_midi** player classes with three synth backends: **TinySoundFont** (SF2), **primesynth**, **ymfmidi**/Opal (OPL3 FM, default bank), plus **VSTi** host on Windows only (`extras\build.mk`). `_MIDISOUNDBANK` selects bank. |
| housekeeping | `snd_update()` called from `MAIN_LOOP` every 16 ms (`libqb.cpp:26180`) — handle cleanup / `_SNDLIMIT` enforcement. |

---

## 6. File I/O and OS services

### 6.1 Files

- `sub_open` (`libqb.cpp:13035-13191`): modes 1 RANDOM (default LEN 128), 2 BINARY, 3 INPUT, 4 OUTPUT, 5 APPEND; ACCESS and LOCK clauses mapped to GFS `access/restrictions/how`. INPUT mode pre-reads byte 0 and sets EOF if it is `CHR$(26)`. Error mapping -5→53, -6→76, -7→70, -8→68, -11→64, -12→54. GW-BASIC form `OPEN "R",#1,...` `13193`.
- `sub_close` `13224`; `FREEFILE` `17344`; `LOF` `16838`; `EOF` `16898`; `LOC` `17053` (QB semantics: records / 128-byte blocks); `SEEK` `16993/17037`; `LOCK/UNLOCK` `22089/22152` (real only on Windows backend).
- Binary/random: `sub_get` `14912`, `sub_get2` (variable-length string) `15059`, `sub_put` `15240`, `sub_put2` `15358`; elements passed as `byte_element_struct{offset,length}` (`common.h:118-121`, `byte_element` `14770`).
- `FIELD`: `libqb\src\file-fields.cpp` (346 lines) — `field_new/add/get/put`, fielded strings are fixed `qbs` linked back to the file via `qbs_field` so LSET/RSET hit the record buffer and CLOSE detaches them.
- Sequential: `sub_file_print` `13380` (tracks `column` for comma zones), `WRITE #` formatting is generated; `INPUT #` item scanning `file_input_chr/skip1310/nextitem` `13284-13379`, `sub_file_input_string` `14365`, numeric readers `14473-14769`; `LINE INPUT #` `17218-17343` (text and BINARY-mode fast path); `INPUT$` `func_input` `17087`.
- `DATA/READ`: `sub_read_string` `14515`, `func_read_float/int64/uint64` `14572-14707` over a compiled-in data blob (`data_offset`, `qbx.cpp:363`).
- `_READFILE$` / `_WRITEFILE` `25717/25745`. `BSAVE/BLOAD` `16735/16781`.

### 6.2 Devices

| Device | Support |
|---|---|
| `SCRN:` | Recognised in `gfs_open` (`gfs.cpp:620-638`); PRINT # redirects to screen printing. |
| `COMn:` | `gfs_open_com_syntax` (`gfs.cpp:186-600`) parses the full QB option string; actual serial I/O only in the Windows backend (`gfs.cpp:735+`). Not available on Linux/macOS. |
| `LPTn:` / `LPRINT` | No device file. `qbs_lprint` (`libqb.cpp:10542`) prints into a hidden 640x960 256-colour page image; `MAIN_LOOP` sends it to the default printer after 10 s idle or at exit via `sub__printimage` (`23391-23486`, **Windows GDI only**; no-op elsewhere / when `DEPENDENCY_PRINTER` off). `LPOS` `10533`. |
| `KYBD:`, `CONS:` | Not found in the runtime (grep) — apparently unsupported. |

### 6.3 SHELL and process

`libqb\src\shell.cpp` (1,529 lines): six near-duplicate functions — `func_shell`, `func__shellhide`, `sub_shell` (wait), `sub_shell2` (`_HIDE`), `sub_shell3` (`_DONTWAIT`), `sub_shell4` (both). Windows: `CreateProcessA` / `ShellExecuteExA` / `system()` with `cmd /c` detection (`cmd_ok`, `cmd_command` `26-164`); Unix: `system()`. Sets `shell_call_in_progress` so MAIN_LOOP idles; leaves fullscreen first. `RUN "file"` `libqb.cpp:20272` (`WinExec`/`system` then exit); `RUN` restart = `sub_run_init` `20247` + generated `runline.txt`; `CHAIN` `qbx.cpp:596-893` (Windows-only body: writes COMMON variables + screen state to `chain###.tmp`, launches target with a magic tag in COMMAND$; `chain_input` `qbx.cpp:543` reads it); `COMMAND$` `command.cpp`; `ENVIRON` `environ.cpp`; `_OS$` `libqb.cpp:23518`; `CLEAR` `qbx.cpp:510`.

### 6.4 Filesystem

`libqb\src\filesystem.cpp` (1,042 lines): `CHDIR`, `MKDIR`, `RMDIR`, `KILL` (wildcards), `NAME`, `FILES` (DOS-style listing with free space), `_FILES$` iterator, `_CWD$`, `_STARTDIR$`, `_DIR$("documents"...)` known folders (two per-OS implementations `118`/`219`), `_FILEEXISTS`, `_DIREXISTS`, `_FULLPATH$`. Path separator fix-up `filepath_fix_directory` (`filepath.cpp:83-131`) converts `\`↔`/` per OS — so BASIC programs with backslashes run on Unix.

### 6.5 Networking

- TCP: in `libqb.cpp:21037-21893`. Raw BSD sockets / Winsock 1.1, **non-blocking** (`ioctlsocket FIONBIO` / `fcntl O_NONBLOCK`), IPv4-style `"TCP/IP:port[:host]"` strings. `connection_new` `21465` handles `_OPENCLIENT`, `_OPENHOST`, `_OPENCONNECTION`; `stream_update` `21378` drains `recv` into a growable buffer on each GET/EOF/LOF; `_CONNECTED` `21849`; `_CONNECTIONADDRESS$` `21742` (uses BASIC-compiled helpers `FUNC__WHATISMYIP`, declared `libqb.cpp:706-707`). Gated by `DEPENDENCY_SOCKETS`.
- HTTP(S): `_OPENCLIENT("HTTP:url")` → `libqb\src\qb_http.cpp` (532 lines) over **libcurl 8.17.0** (vendored and built for Windows with Schannel, `HTTP_ONLY`; system libcurl on Linux/macOS). curl-multi on its own thread, responses buffered in `libqb_buffer` chains; `_STATUSCODE` `libqb.cpp:16968`. `qb_http-stub.cpp` when `DEP_HTTP` is off.

### 6.6 Other services

| Feature | Location / library |
|---|---|
| `_SCREENIMAGE` | `libqb.cpp:22216-22313`; **Windows GDI only**, other OSes return an error/blank (`22308-22311`); glut-emu TODO lists desktop capture. |
| `_SCREENCLICK`, `_SCREENPRINT` | `22315`, `22772-23383`; Windows `SendInput`, macOS CGEvent (large VK tables); Linux path not confirmed. |
| Dialogs | `parts\gui\gui.cpp` (427 lines) over **tinyfiledialogs 3.8.9**: `_MESSAGEBOX`, `_INPUTBOX$`, `_OPENFILEDIALOG$`, `_SAVEFILEDIALOG$`, `_SELECTFOLDERDIALOG$`, `_COLORCHOOSERDIALOG`, `_NOTIFYPOPUP`; also runtime error alerts (`gui_alert`). |
| Compression | `parts\data\compression.cpp` (83 lines) over **miniz 11.3.1** (`miniz.h:289`): `_DEFLATE$`, `_INFLATE$`, `_ADLER32`, `_CRC32`. |
| Base64 | `parts\data\encoding.cpp` over **modp_b64** (stringencoders). |
| `_MD5$` | `parts\video\font\hashing.cpp` using FreeType's MD5. |
| Image load | `parts\video\image\image.cpp` (1,086 lines): **stb_image 2.30** (PNG, JPEG, BMP, TGA, GIF, PSD, HDR, PIC, PNM), **nanosvg** (SVG), **qoi**, **sg_pcx**, **sg_curico** (ICO/CUR), **tiny_webp**; fallbacks tried in that order (`image.cpp:324-341`); file or memory source; optional 8-bit palette conversion and pixel-art scalers **hqx / mmpx / sxbr**. |
| Image save | `_SAVEIMAGE`: **stb_image_write 1.16** (PNG, BMP, TGA, JPG, HDR), qoi, **jo_gif**, ICO via sg_curico. |
| Fonts | `parts\video\font\font.cpp` (1,509 lines) over **FreeType 2.14.1** (flattened vendored build). `FontManager` caches glyph bitmaps per font; monospace / auto-mono / Unicode / dontblend flags; `_UPRINTSTRING`, `_UPRINTWIDTH`, `_UFONTHEIGHT`, `_ULINESPACING`, `_UCHARPOS` for UTF-8/16/32 text. |
| Logging | `libqb\src\logging\` — levels/scopes, env-configured handlers, stack traces with own PE/ELF symbol resolver (`mingw\`, `unix\`). `_LOGTRACE/INFO/WARN/ERROR`, `_LOGMINLEVEL`. |
| Date/time | `datetime.cpp`: `TIMER` (18.2 Hz quantised and squeezed through a `float` when no accuracy arg, `46-73`), `DATE$`, `TIME$`, `_DELAY`, `_LIMIT`. |
| RNG | `libqb.cpp:12861-12915`: exact QB45 24-bit LCG `seed = (seed*16598013 + 12820163) & 0xFFFFFF`, initial 327680, QB-compatible `RANDOMIZE n` seed folding and `RND(negative)` reseed. |

---

## 7. Inventory

### 7.1 `libqb\src` modules

| File | Lines | Responsibility | Key exports |
|---|---|---|---|
| `qbs.cpp` | 676 | String heap, descriptors, compare, case | `qbs_new*`, `qbs_set`, `qbs_add`, `qbs_free`, `qbs_maketmp`, `qbs_ucase/lcase/left/right`, `qbs_equal…`, `qbs_asc`, `func_chr` |
| `qbs_cmem.cpp` | 230 | Strings inside DBLOCK | `qbs_create_cmem`, `qbs_copy_cmem`, `qbs_move_cmem`, `qbs_remove_cmem`, `qbs_new_fixed_cmem` |
| `qbs_str.cpp` | 255 | `STR$` (QB formatting) | `qbs_str` × 11 overloads |
| `qbs__tostr.cpp` | 188 | `_TOSTR$` (`%.*G`) | `qbs__tostr` × 11 |
| `qbs_val.cpp` | 293 | `VAL` incl. &H/&O/&B, D/E/F exponents | `qbs_val<T>` |
| `qbs_mk_cv.cpp` | 530 | MKI$/CVI… , MBF conversion | `*2string`, `string2*`, `func_mksmbf/mkdmbf/cvsmbf/cvdmbf` |
| `string_functions.cpp` | 363 | LSET/RSET/SPACE$/STRING$/INSTR/MID$/trim/compare | `sub_lset`, `func_instr`, `func__instrrev`, `sub_mid`, `func_mid`, `qbs_ltrim/rtrim/_trim`, `func__str_compare` |
| `hexoctbin.cpp` | 285 | HEX$/OCT$/_BIN$ | `func_hex`, `func_oct`, `func__bin` (+ `_float`) |
| `error_handle.cpp` | 507 | Error state, dialogs, stack check | `error`, `fix_error`, `clear_error`, `libqb_check_stack`, `func__errorline…` |
| `memblock.cpp` | 332 | `_MEM` | `func__mem`, `func__memnew`, `sub__memfree`, `sub__memcopy`, `sub__memfill*`, `new_mem_lock` |
| `array-copy.cpp` | 954 | `_ARRAYCOPY` | `qb64_array_copy_{1d,nd}_*_{string,fixed}` |
| `gfs.cpp` | 1,186 | File backend, COM syntax | `gfs_open/close/read/write/lof/setpos/lock…` |
| `file-fields.cpp` | 346 | FIELD | `field_new/add/get/put/free/update`, `lrset_field` |
| `filesystem.cpp` | 1,042 | Dir/file statements | `sub_chdir/mkdir/rmdir/kill/name/files`, `func__files/_dir/_cwd/_fullpath…` |
| `filepath.cpp` | 161 | Path helpers | `filepath_fix_directory`, `filepath_split/join` |
| `shell.cpp` | 1,529 | SHELL variants | `func_shell`, `func__shellhide`, `sub_shell[2-4]` |
| `command.cpp` | 64 | COMMAND$ | `func_command`, `func__commandcount`, `command_initialize` |
| `environ.cpp` | 104 | ENVIRON | `func_environ` ×2, `sub_environ`, `func__environcount` |
| `datetime.cpp` | 287 | Clock, TIMER, DATE$/TIME$, _LIMIT/_DELAY | `GetTicks`, `func_timer`, `sub__limit`, `sub__delay`, `Sleep` (posix) |
| `console.cpp` | 278 | `_CONSOLE`, console input/mouse, `_ECHO` | `sub__console`, `func__getconsoleinput`, `func__cinp`, `sub__echo` |
| `keyboard.cpp` | 2,510 | All keyboard translation and buffers | `GLUT_KEYBOARD_*`, `qbs_inkey`, `func__keyhit`, `func__keydown`, `sub__keyclear`, lock-key funcs |
| `key-events.cpp` | 119 | ON KEY / KEY n | `onkey_setup`, `sub_key`, `onkey_assign_binding` |
| `mouse.cpp` | 693 | Mouse queue and API | `GLUT_MOUSE_*`, `func__mouseinput/x/y/button/wheel`, `sub__mouseshow/hide/move` |
| `graphics.cpp` | 2,915 | HSB colour, `_DEPTHBUFFER`, `_MAPTRIANGLE` | `func__hsb32…`, `sub__depthbuffer`, `sub__maptriangle` |
| `glut-emu.cpp` | 2,665 | GLFW wrapper with cross-thread message queue | `GLUTEmu_*` (≈70 functions) |
| `window-gui.cpp` / `window-console.cpp` | 423 / 268 | Window policy / stubs | `sub__fullscreen`, `sub__resize`, `sub__title`, `sub__screenmove`, drop files… |
| `main-thread-gui.cpp` / `-console.cpp` | 155 / 32 | Thread bring-up, exit | `libqb_start_main_thread`, `libqb_glut_presetup`, `libqb_exit`, `libqb_is_glut_thread` |
| `threading*.cpp` | 31 + 99 + 103 | Threads, mutex, condvar, completion | `libqb_thread_*`, `libqb_mutex_*`, `completion_*` |
| `qblist.cpp` | 148 | Generic handle list | `list_new`, `list_add`, `list_get`, `list_remove` |
| `buffer.cpp` | 89 | Chunk FIFO (HTTP) | `libqb_buffer_*` |
| `qb_http.cpp` / `-stub.cpp` | 532 / 63 | curl HTTP | `libqb_http_*` |
| `logging\*` | ~1,000+ | Logging and stack traces | `libqb_log*` |

Header-only runtime: `rounding.h` (`qbr*`, `func_cint/clng/csng/cdbl/round`), `qbmath.h` (`func_log/sqr/exp/abs/sgn/fix`, `pow2`), `extended_math.h` (trig extras, `_PI`, clamp), `bitops.h` (`_SHL/_SHR/_ROL/_ROR`, bit ops, `getbits/setbits`), `unicode.h`, `ring-buffer.h`.

### 7.2 What is still in `libqb.cpp` (approximate line ranges)

| Range | ~Lines | Group |
|---|---|---|
| 1-700 | 700 | Includes, globals, hardware image creation, modal lock, CP437 table, `convert_unicode` |
| 716-2355 | 1,640 | Embedded data: 8x8 font, 8x16 font, 256 and EGA palettes |
| 2358-3027 | 670 | UTF-16 text conversion, blend tables, palettes, `pset`, image table (`newimg`…`imgnew`), hardware command GC |
| 3029-4652 | 1,625 | `sub__putimage` |
| 4653-4870 | 220 | `selectfont`, CPU struct |
| 4871-5835 | 965 | x86 emulator (`sib`, `rm8/16/32`, `cpu_call`) |
| 5836-6184 | 350 | Misc globals, `end`, `mem_static`, `cmem_dynamic`, DEF SEG/PEEK/POKE, `qbg_*` globals |
| 6185-6844 | 660 | `lineclip`, PALETTE, COLOR, default colours, `validatepage` |
| 6845-8242 | 1,400 | `qbg_screen`, PCOPY, WIDTH |
| 8243-10127 | 1,885 | Drawing: boxfill/line, PAINT ×4, tile helpers, CIRCLE, POINT, PSET |
| 10128-11633 | 1,505 | Text output: `printchr`, `newline`, `tab`, LPRINT, `qbs_print`, WINDOW, VIEW PRINT, VIEW, CLS, LOCATE |
| 11634-12789 | 1,155 | `hexoct2uint64`, `qbs_input` |
| 12790-12940 | 150 | `_BLINK`, OUT, RANDOMIZE, RND, `_FPS` |
| 12941-13536 | 595 | Generic get/put, OPEN, CLOSE, file input scanning, `sub_file_print` |
| 13537-14769 | 1,235 | Number parsing (INPUT/READ/INPUT #) |
| 14770-14911 | 140 | `byte_element`, CALL INTERRUPT |
| 14912-15418 | 505 | File GET/PUT |
| 15419-16155 | 735 | Graphics GET/PUT |
| 16156-17433 | 1,280 | CSRLIN/POS, SLEEP, LBOUND/UBOUND, INP, WAIT, TAB/SPC, PMAP, SCREEN(), BSAVE/BLOAD, LOF/EOF/SEEK/LOC, INPUT$, LINE INPUT #, FREEFILE, CALL ABSOLUTE, INT 33h |
| 17434-18284 | 850 | Image API (`_NEWIMAGE`…`_COPYPALETTE`) |
| 18285-19016 | 730 | `_PRINTSTRING`, `_PRINTWIDTH`, font API, `_PRINTMODE`, `matchcol` |
| 19017-19280 | 265 | `_RGB`/`_RGBA`/component functions |
| 19281-19356 | 75 | `sub_end` |
| 19357-20246 | 890 | PRINT USING (`print_using` ~600 lines + 5 numeric front-ends) |
| 20247-20364 | 120 | RUN, `_ICON`, `_DISPLAY`/`_AUTODISPLAY` |
| 20365-21036 | 670 | DRAW |
| 21037-21894 | 860 | TCP, streams, connections |
| 21895-22215 | 320 | `_EXIT`, `display_now`, CHAIN screen state, LOCK/UNLOCK |
| 22216-23489 | 1,275 | `_SCREENIMAGE`, `_SCREENCLICK`, `_SCREENPRINT`, `_PRINTIMAGE` |
| 23490-24098 | 610 | `_MAPUNICODE`, `_OS$`, KEY bar, PALETTE USING, STICK/STRIG, `_MEMIMAGE` |
| 24099-24236 | 140 | GL idle, `_DISPLAYORDER`, `_GLRENDER` |
| 24237-25309 | 1,075 | Hardware rendering helpers (2D env, state setters, put/tri2d/tri3d) |
| 25310-25715 | 405 | `GLUT_DISPLAY_REQUEST` |
| 25717-25806 | 90 | `_READFILE$`, `_WRITEFILE`, scaled size, cygwin pipe |
| 25807-26276 | 470 | `main`, `MAIN_LOOP` |
| 26278-27042 | 765 | `display()` |
| 27043-27119 | 75 | dead SDL comment, `GLUT_EXIT_FUNC` |

### 7.3 Entry-point count

Unique externally callable names found by signature grep over `libqb.cpp`, `qbx.cpp`, `libqb\src`, `libqb\include` and first-party `parts` glue:

| Prefix | Unique names |
|---|---|
| `func_*` | ~262 |
| `sub_*` | ~161 |
| `qbs_*` | ~44 (64 definitions with overloads) |
| `*2string` / `string2*` | 30 |
| `gfs_*` | 23 |
| `qb64_array_copy_*` | 12 |
| `print_using*` | 6 |
| `qbg_*` | 7 |
| other helpers called by generated code (`evnt`, `error`, `byte_element`, `mem_static_*`, `cmem_dynamic_*`, `field_*`, `ontimer_setup`, `onkey_setup`, `onstrig_setup`, `call_*`, `swap_*`, `array_check`, `check_lbound`, `n_*`, `tab`, `newline`, etc.) | ~60 |

**Estimate: about 600 runtime entry points** (roughly 650-700 symbols counting C++ overloads), plus the generated `_gl*` wrappers in `gl_helper_code.h` (not counted; several hundred). For cross-checking, `source\subs_functions\subs_functions.bas` has ~455 lines mentioning `regid`.

Calling-convention facts a rewrite must note: optional arguments are passed with a trailing `int32 passed` bitmask (each function documents bits in a comment, e.g. `libqb.cpp:13038-13043`, `3032-3036`); many functions are C++ overloads selected by the code generator; math/rounding helpers are `static inline` in headers and inlined into user code.

---

## 8. Platform abstraction

### 8.1 OS/compiler macros (`libqb-common.h:19-58`)

`QB64_WINDOWS`, `QB64_LINUX`, `QB64_MACOSX`, `QB64_UNIX`, `QB64_BACKSLASH_FILESYSTEM`, `QB64_MICROSOFT` (MSVC; effectively unsupported), `QB64_GCC`, `QB64_MINGW`, `QB64_32`/`QB64_64`, `QB64_NOT_X86`, `QB64_ARM`. Any other OS is a hard `#error`. `_WIN32_WINNT` 0x0600 (Vista+).

### 8.2 How differences are handled

There is no single platform layer; three patterns coexist:

1. **Per-platform source files chosen by the makefile** (cleanest): `threading-{windows,posix}.cpp`, `main-thread-{gui,console}.cpp`, `window-{gui,console}.cpp`, `qb_http{,-stub}.cpp`, `logging\{mingw,unix}\symbol.cpp`, libstem gamepad, clip.
2. **Third-party portability libraries**: GLFW (window/input/GL context), miniaudio, tinyfiledialogs, clip, curl, FreeType.
3. **Inline `#ifdef`** in `libqb.cpp` (~57 sites; list from grep): console attach (`25810`), exe-dir discovery (`25983-26012`), console size queries (`7533`, `11486`, `16157`, `16176`, `17959`, `17998`), SLEEP on console (`16201`), sockets (`21037-21405`), RUN/CHAIN (`20287`, `qbx.cpp:600`), `_SCREENIMAGE` (`22218`), `_SCREENCLICK`/`_SCREENPRINT` (`22317-23163`), printer (`23393`), `_OS$` (`23519-23529`), gamepad poll thread (`24100`, `26131`), X11 `XInitThreads` (`25826`). Also `gfs.h` (`GFS_WINDOWS` vs `GFS_C`), `filesystem.cpp`, `shell.cpp`, `console.cpp`, `error_handle.cpp:104-133`, `rounding.h:9` (x87 asm vs `nearbyint`).

Windows-only features (silently degraded elsewhere): COM ports, file LOCK, LPRINT/`_PRINTIMAGE`, `_SCREENIMAGE`, CHAIN, most `_CONSOLE` input features, `logical_drives`, VSTi MIDI, `_WINDOWHANDLE` semantics, `set_foreground_window`.

`common.h` supplies `ZeroMemory` for non-Windows (`122-126`), `libqb.cpp` stubs `AllocConsole/FreeConsole` (`693-701`), `datetime.h` supplies `Sleep()`.

### 8.3 `DEPENDENCY_*` macros

Set by the root `Makefile` from `DEP_*` variables that the compiler derives from features used in the BASIC program (Makefile lines ~301-473).

| Macro | Gates |
|---|---|
| `DEPENDENCY_CONSOLE_ONLY` | No GUI: undefines `QB64_GUI` (`common.h:22`); selects console main-thread/window files; `sub_end` hides screen (`libqb.cpp:19291`); `_ICON` body (`20322`); mouse stubs (`mouse.cpp`, 5 sites). Also (ab)used as a compile flag for all `parts` glue. |
| `QB64_GUI` (derived) | GL includes; real `new_hardware_img`; GL idle limiter; entire hardware renderer and `GLUT_DISPLAY_REQUEST` (`24226-25715`). |
| `DEPENDENCY_GL` | Program has `SUB _GL`: includes `gl_helper_code.h` (`qbx.cpp:55`), calls `SUB__GL()` (`libqb.cpp:25432`), modal lock counter (`444-477`), render-without-new-frame path (`25369`). |
| `DEPENDENCY_SOCKETS` / `_NO_SOCKETS` | Winsock init and TCP code (11 sites, `21050-21405`); default **on** (`common.h:2-4`). |
| `DEPENDENCY_PRINTER` / `_NO_PRINTER` | `<winspool.h>` include; `sub__printimage` real vs stub (`23381`). Default on. |
| `DEPENDENCY_ICON` / `_NO_ICON` | `sub__icon` existence (`20316`); icon resource (`DEP_ICON_RC`). Default on. |
| `DEPENDENCY_SCREENIMAGE` / `_NO_SCREENIMAGE` | `func__screenimage` body (`22216-22313`). Default on. |
| `DEPENDENCY_DEVICEINPUT` | Gamepad init/poll/shutdown (`26100`, `26130`, `24101`, `26271`); links game_controller lib. |
| `DEPENDENCY_AUDIO_MINIAUDIO` | `<mmsystem.h>` include (`common.h:81`); links audio lib vs `stub_audio.o`. |
| `DEPENDENCY_IMAGE_CODEC` | Makefile only (links image lib); not referenced in the sources scanned. |
| Makefile-only `DEP_FONT`, `DEP_ZLIB`, `DEP_HTTP`, `DEP_CONSOLE`, `DEP_EMBED`, `DEP_ICON_RC` | Choose real lib vs stub object (`stub_font.cpp`, `qb_http-stub.cpp`), console subsystem link flag, embedded files. |

Net effect: the runtime is **recompiled per program** with a different macro set, and unused subsystems are replaced by link-time stubs. (Build details belong to the build-system report.)

### 8.4 `parts\` summary

| Part | First-party glue | Vendored | Version | Backs | Build flags (brief) |
|---|---|---|---|---|---|
| `audio` | `audio.cpp` (3,913), `framework.h`, `extras\*_ma_vtable.cpp` (5 files, ~2,700), `stub_audio.cpp` | miniaudio | 0.11.25 | all sound | `-O3`; C++ glue with `-DDEPENDENCY_CONSOLE_ONLY` |
| `audio\extras` | — | stb_vorbis; libxmp-lite (`-DLIBXMP_CORE_PLAYER -DLIBXMP_STATIC`); hivelytracker `hvl_replay`; radv2/Opal; qoa; tinysoundfont; primesynth; ymfmidi; libmidi; foo_midi (+VSTi on Windows) | not stamped (not checked individually) | OGG, MOD/XM/S3M/IT, AHX/HVL, RAD, QOA, MIDI family | `-O3` |
| `core` | none (`gl_header_for_parsing` = generator input + generated `gl_helper_code.h`) | GLFW; GLAD | 3.5.1; 2.0.8 (generated 2026-02-14) | window, input, GL context, `_gl*` | `-D_GLFW_WIN32` / `_GLFW_X11` (Wayland sources compiled too) / `_GLFW_COCOA`, `-w` |
| `data` | `compression.cpp` (83), `encoding.cpp` (27) | miniz; modp_b64 | 11.3.1; n/a | `_DEFLATE$/_INFLATE$`, `_CRC32/_ADLER32`, `_BASE64*` | `-O3` |
| `gui` | `gui.cpp` (427) | tinyfiledialogs | 3.8.9 | dialogs, error alerts, BEEP | `-O2` |
| `input\game_controller` | `game_controller.cpp` (241) | libstem_gamepad | n/a | `_DEVICES`, STICK/STRIG | `-O2` |
| `network\http` | (glue is `libqb\src\qb_http.cpp`) | libcurl | 8.17.0 | `_OPENCLIENT("HTTP...")` | Windows: `-DHTTP_ONLY -DUSE_SCHANNEL -DUSE_WINDOWS_SSPI -DCURL_STATICLIB`; others `-lcurl` |
| `os\clipboard` | `clipboard.cpp` (389) | clip (dacap) | n/a | `_CLIPBOARD$`, `_CLIPBOARDIMAGE` | `-DCLIP_ENABLE_IMAGE=1`; Linux `-DHAVE_XCB_XLIB_H -DHAVE_PNG_H -lpng` |
| `video\font` | `font.cpp` (1,509), `hashing.cpp` (39), `stub_font.cpp` | FreeType (flattened) | 2.14.1 | `_LOADFONT`, `_U*` text, `_MD5$` | `-DFT2_BUILD_LIBRARY`, `-O3 -w` |
| `video\image` | `image.cpp` (1,086) | stb_image 2.30, stb_image_write 1.16, nanosvg, qoi, jo_gif, tiny_webp, sg_pcx, sg_curico, pixelscalers (hqx, mmpx, sxbr) | as listed | `_LOADIMAGE`, `_SAVEIMAGE` | `-O3` |

---

## 9. Tech debt, hazards, and what must be preserved

### 9.1 Behaviours a rewrite must preserve (compatibility contract)

**Numeric**
- **Rounding = x87 `fistp` round-half-to-even** everywhere a float becomes an integer (`qbr`, `qbr_float_to_long`, `qbr_double_to_long`, `rounding.h:48-90`). Used by CINT/CLNG, implicit assignment, graphics coordinates, palette maths. Range checks produce error 6 at exactly `±32767.5`-style thresholds (`rounding.h:127-216`).
- **`_FLOAT` = 80-bit `long double`**; QBMAIN sets x87 precision control to extended (`fpu_reinit`). On non-x86 (ARM) `long double` semantics differ — existing behaviour is already platform-dependent; decide explicitly.
- `STR$`/PRINT number formatting (`qbs_str.cpp`): leading space for non-negative, SINGLE 7 significant digits, DOUBLE 16, switch to `E+nn`/`D+nn` exponent form by QB's rule `exponent <= 6 && exponent-digits >= -8` (`qbs_str.cpp:104`), at least two exponent digits, no leading zero before the decimal point (`.5`). Implemented by post-processing `sprintf("% .6E")` — result depends on the C library's rounding.
- `VAL` (`qbs_val.cpp`): skips blanks/tabs anywhere, accepts `D/E/F` exponent letters, `&H/&O/&B`, stops at first invalid char.
- **PRINT USING** (`libqb.cpp:19357-20246`): full QB mini-language (`#`, `.`, `,`, `+`, `-`, `$$`, `**`, `**$`, `^^^^`/`^^^^^`, `!`, `\ \`, `&`, `_` escape, `%` overflow prefix), digit-buffer algorithm with re-pass on rounding carry.
- `RND`/`RANDOMIZE` exact LCG and seed folding (§6.6). `RUN` resets the seed (`20266`).
- `TIMER` without args: quantised to 1/18.2 s and passed through `float`.
- MBF conversions `MKSMBF$/CVSMBF` etc. (`qbs_mk_cv.cpp:44-290`).
- `HEX$/OCT$` width behaviour by operand type (`hexoctbin.cpp`).
- `^` domain error for negative base with non-integer exponent (`qbmath.h:59-67`); `LOG`, `SQR` → error 5; `EXP` overflow thresholds 88.02969 / 709.782712893.

**Strings / memory**
- `qbs_set` fixed-length pad/truncate; `LSET/RSET`; `MID$` statement clamping; byte strings, CP437 semantics, no encoding.
- cmem layout: DBLOCK at segment &H50, SCREEN 13 at &HA000, SCREEN 0 at &HB800, BIOS keyboard buffer and tick count — programs PEEK/POKE these.
- `DEF SEG`/PEEK/POKE range errors; `VARPTR$` 3-byte format; INT 33h mouse; DAC port I/O and 6-bit↔8-bit conversion constants; `&H3DA` retrace bit; `&H60` scancodes.
- `_MEM` struct layout, error numbers 300-313, handle conventions (negative image handles, values fit in SINGLE).

**Screen / text**
- Mode table (§3.1) incl. default fonts, default colours, per-mode palettes, SCREEN 10 pseudo-palette, page sharing of palette.
- PRINT semantics: comma zones (14 columns), deferred wrap ("holding cursor"), scroll within VIEW PRINT, control characters, `_CONTROLCHR`, `LOCATE` cursor shape, blink attribute, WIDTH switching rules and the 80x50 auto switch.
- "Press any key to continue" on END (not SYSTEM); unhandled-error dialog text and "Continue?" behaviour; exit code handling (`exit_code`).
- GET/PUT array binary format per mode; BSAVE/BLOAD 7-byte header; PAINT tiling bit layout; DRAW language; CIRCLE aspect defaults; LINE style bits; WINDOW/VIEW transforms.
- INKEY$ two-byte codes, `_KEYHIT` code space, ON KEY(n) numbering.

**Files**
- OPEN defaults (RANDOM LEN=128), CHR$(26) EOF handling on INPUT files, LOC/LOF/EOF semantics, FIELD aliasing, INPUT # parsing (quotes, commas, CR/LF), error-number mapping.
- Start-up `chdir` to executable directory; `\`→`/` path fixing on Unix.

**Events / timing**
- Events and errors are only serviced at statement boundaries (`qbevent` polling) and inside `_LIMIT`/`_DELAY`/`SLEEP`/INPUT waits. ON TIMER catch-up rule (`qbx.cpp:1374-1385`). `SLEEP` broken by key or timer event.
- Auto-display ~31 fps on a separate thread vs manual `_DISPLAY`; `_DISPLAYORDER` layering; hardware-image commands bound to the next `_DISPLAY`.

### 9.2 Hazards / tech debt

1. **Error model without unwinding.** `error()` returns; every function must early-out; forgetting to means operating on bad state. Handled errors recurse into `QBMAIN` (`error_handle.cpp:430-431`) — unbounded native stack growth in programs that trap many errors.
2. **`static` locals everywhere** (892 in `libqb.cpp`) as a 2000s-era optimisation → functions are non-reentrant. Dangerous because `SUB _GL` and ON-event handlers do re-enter runtime functions (GL thread vs QBMAIN are serialised only by the display-lock handshake; timer GOSUBs re-enter on the same thread mid-`evnt`).
3. **Data races by design**: `lock_display`, `autodisplay`, `display_frame[].state`, `exit_ok`, `stop_program`, `qbevent`, `ontimer[]`, `img[]` (realloc'd on QBMAIN while `display()` reads `display_page`), palettes and page memory are all shared across threads without atomics. `display()` copies pixels while QBMAIN draws (tearing is accepted). Busy-wait spins (`Sleep(0)`, bare `while` in `stop_timers`).
4. **Moving string heap**: any allocation may relocate all string data; code holding `chr` pointers across calls is a latent bug class. `qbs_add` aliasing rules are subtle. 32 bytes of slack per string; descriptors and old arenas are never returned to the OS. `uint32` heap offsets cap total string space at 4 GB and `len` at 2 GB.
5. **Global mutable state** for everything (hundreds of globals shared via `extern` between `libqb.cpp`, `qbx.cpp` and modules; headers carry `REFACTOR_TODO`/`FIXME` notes, e.g. `graphics.h:220-242`, `cmem.h`, `qbs.h:92`).
6. **Monster functions with `goto`** (628 gotos): `sub__putimage` 1,600 lines, `sub__maptriangle` ~2,600, `qbs_input` ~1,050, `display` ~750, `qbsub_width` ~750, `print_using` ~600, `qbg_screen` ~600, `keyboard_keydown` ~440. Near-duplicate code: `imgframe`/`imgrevert`, six SHELL variants, four PAINTs.
7. **Fixed-function OpenGL + GLU** in the presentation path and exposed to users via `SUB _GL` (compat profile). A core-profile/modern backend cannot keep `_gl*` user code working without a compatibility context.
8. **Memory footprint constants**: `cmem` 1.1 MB, `cmem_dynamic_link[147137]` + free list (~6.5 MB), `onstrig` 65,536 entries, `keyon[65536]`, 16 MB blend LUT on first 32-bit image, 65,536-entry rings.
9. **Windows-only features with silent no-ops** elsewhere (§8.2) and unsupported devices (`KYBD:`, `CONS:`, real `LPTn:`).
10. **Tiny x86 emulator**: only supports the canonical mouse stub; anything else pops an "X86 Error" dialog.
11. **Integer division by zero is fatal** (SIGFPE → critical error), unlike QB45's trappable error 11.
12. **`end()` parks the BASIC thread forever**; shutdown depends on the `exit_ok` bit dance; `exit()` runs with other threads alive.
13. **`MAIN_LOOP` uses `Sleep(15)+Sleep(1)`** for pacing; on Windows timer granularity makes the "32 fps" nominal. The emulated retrace bit is a 1 ms pulse per 16 ms.
14. **printf-dependent float formatting** and `long double` width differences across compilers (MinGW vs MSVC vs ARM).
15. **Winsock 1.1, IPv4-oriented, hand-rolled non-blocking sockets**; `_CONNECTIONADDRESS$` depends on BASIC-side helper functions linked into every program.
16. Dead code and stale comments: SDL remnants (`27044-27114`), unused scancode table, `generic_put/get` "largely redundant" (`12941`), `array_ok` "kept to compile legacy versions", "FreeGLUT" naming over GLFW, `GLFW_TODO` markers (maximise/minimise handling, console terminal emulator hook).
17. `list` implementation leaks old blocks by design (`qblist.h:19-21`), and thread safety covers add/remove but not `list_get` vs growth.
18. Type aliases as macros (`#define int32 int32_t`, `os.h`) leak into every including file and conflict with Windows headers (`common.h:44-48`).

### 9.3 Reusable as-is vs redesign

**Reusable with little change (already modular, tested by upstream, clear APIs)**
- `threading-*`, `completion`, `mutex`, `condvar`, `ring-buffer.h`, `buffer.cpp`.
- `glut-emu.cpp` as the GLFW façade with its cross-thread message queue (rename; it is not GLUT).
- `parts\audio\audio.cpp` + decoder vtables (self-contained class, only needs `qbs`, `mem_block`, logging).
- `parts\video\image\image.cpp`, `parts\video\font\font.cpp`, `parts\gui\gui.cpp`, `parts\data\*`, `parts\os\clipboard\clipboard.cpp`, `parts\input\game_controller\*`, `qb_http.cpp`.
- `filesystem.cpp`, `filepath.cpp`, `environ.cpp`, `command.cpp`, `datetime.cpp`, `logging\`.
- Pure algorithm units worth lifting verbatim to guarantee bit-exactness: `rounding.h`, `qbmath.h`, `bitops.h`, `extended_math.h`, `qbs_str.cpp`, `qbs_val.cpp`, `qbs_mk_cv.cpp`, `hexoctbin.cpp`, `string_functions.cpp`, `print_using` (`libqb.cpp:19357-20246`), RND (`12861-12915`), number parsers (`13537-14769`), `array-copy.cpp`, `memblock.cpp`.
- Data tables: CP437 map, charsets, palettes, keyboard scancode tables.
- All vendored libraries.

**Reusable as reference behaviour but should be re-housed/refactored**
- Software rasterisers (LINE/CIRCLE/PAINT/DRAW/GET/PUT/`_PUTIMAGE`/`_MAPTRIANGLE`): keep the algorithms for pixel-exactness, but pass the target image explicitly instead of global `write_page`, drop statics.
- Text engine (`printchr`/`qbs_print`/`qbs_input`/`display()` text renderer).
- `keyboard.cpp` (correct but large; five output sinks could become subscribers of one event stream).
- `gfs.cpp` (two backends; unify on one portable backend with locking and serial support behind an interface) and file statement layer.
- `mouse.cpp`, `window-gui.cpp`.

**Needs redesign**
- Thread/lock architecture (flag handshakes → proper frame hand-off, atomics/condvars; define exactly which thread owns image memory).
- Error handling (structured unwind or explicit result propagation; no recursive `QBMAIN`); shutdown path.
- String memory manager (moving heap + descriptor lists) — unless the new compiler's codegen is designed around the same `qbs` ABI.
- Conventional-memory emulation: keep observable layout, but isolate behind an interface and allocate lazily; reconsider the x86 interpreter scope.
- Global image/page/font tables and handle scheme (keep numeric handle conventions, change storage).
- Hardware-image command queue and fixed-function GL renderer.
- `DEPENDENCY_*` per-program recompilation model and the `qbx.cpp` + `temp\*.txt` textual-include contract between compiler and runtime (owned by the build/codegen studies, but it shapes every runtime entry point: `passed` bitmasks, overload selection, inline helpers, `byte_element_struct`, array descriptor layout).
- SHELL/RUN/CHAIN process control; printing; `_SCREENIMAGE`/`_SCREENCLICK`/`_SCREENPRINT` platform coverage.

---

## 10. Not determined / to verify

- Full array-descriptor slot map (slots 1, 3, and `[4k+2]`), and the `_MEM`-on-array lock slot: defined by the code generator, only partially visible from the runtime.
- Exact INPUT redo behaviour and messages (`qbs_input` body sampled only).
- Where the 8x14 font glyphs come from (`selectfont`/`printchr` internals not traced).
- Linux implementations of `_SCREENCLICK`/`_SCREENPRINT` and non-Windows `_SCREENIMAGE` result (appears unsupported).
- ON COM / ON PEN / ON PLAY, `PEN`, `KYBD:`/`CONS:`/`LPTn:` device files: not found in the runtime by grep; compiler-side behaviour unchecked.
- Versions of the smaller vendored libraries (clip, libstem_gamepad, libxmp-lite, tinysoundfont, primesynth, ymfmidi, libmidi/foo_midi, nanosvg, qoi, jo_gif, tiny_webp, hqx/mmpx/sxbr, modp_b64, stb_vorbis) are not stamped in files I inspected.
- Internals of `hardware_img_put/tri2d/tri3d`, `prepare_environment_2d`, the middle of `display()` (text glyph rendering, 32-bit path), PAINT/CIRCLE/DRAW/GET/PUT bodies, `sub__maptriangle`, `keyboard_keydown`, and `audio.cpp` PSG/MML details were outlined, not read in full.
- Contents of generated `internal\temp\*.txt` (directory holds only `temp.bin` in this checkout), so the exact generated-code/runtime protocol (event checks, RESUME tables, `SUB__GL` wrapper) is inferred from `qbx.cpp` include points.
