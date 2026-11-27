// The run-time part of a call-trace build (OpenSpec change call-trace; call_trace.rs writes the tables above this
// block into global.txt). The program keeps a stack of frames, one per procedure that is running. libqb's report of
// an untrapped runtime error calls qbr_ct_hook through libqb_error_call_stack, and puts the text it returns after
// the error's text.

// A frame: the procedure's number (0 the main module, 1 an error handler, K + 1 procedure K), the site it was called
// from (0 not known, 1 an event, else an entry of qbr_ct_at), and libqb's next_return_point when it started, so that
// the GOSUBs on libqb's return stack above it are its own (design D4).
struct qbr_ct_frame {
    uint32 proc, site, gosubs;
};

// The state of one thread (design D3): SUB _GL and what it calls run on libqb's display thread, beside the program.
// site is the site of the statement running, set before a statement that calls a procedure. main is set where the
// main module runs, so only that thread has the main module's row and the GOSUB rows. lost counts the innermost
// frames that could not be kept when the stack could not grow. The last three are scratch for the macros below,
// which declare no variable of their own.
struct qbr_ct_state {
    qbr_ct_frame *frames;
    uint32 depth, cap, lost, site;
    uint8 main;
    uint32 new_cap, popped;
    qbr_ct_frame *grown;
};
static thread_local qbr_ct_state qbr_ct;

// The frames are pushed and popped by macros, not functions, for two reasons (design, "Found while building"). The
// default build has no -O2, so a function would not be inlined, and each call of a procedure would pay for two more
// calls. And every function of the program's C++ has line information, so a debugger's step into a SUB would stop
// in it; a macro's code stands on the line of the BASIC statement it is in, and realloc has no line information.

// They declare no variable: a scope with a variable is a lexical block in the debug information, and lldb places a
// breakpoint once per block of a line, so a step from the first place stopped at the next (`cli::dap_stepping`). A
// procedure takes its thread's state once, as `qbr_ct_state &qbr_ct_s = qbr_ct;` at its start, and the macros work
// on the state S they are given.

// A frame of procedure p, called from site s, onto S. The stack grows when it is full; when it cannot, the frame is
// only counted (lost), and so are those pushed after it. The stack has one slot more than its capacity, for the
// pop's read below the depth.
#define qbr_ct_push_to(S, p, s)                                                                                        \
    if ((S).depth == (S).cap) {                                                                                        \
        (S).new_cap = (S).cap ? (S).cap * 2 : 256;                                                                     \
        (S).grown = NULL;                                                                                              \
        if (!(S).lost && (S).new_cap > (S).cap)                                                                        \
            (S).grown = (qbr_ct_frame *)realloc((S).frames, ((size_t)(S).new_cap + 1) * sizeof(qbr_ct_frame));         \
        if ((S).grown) {                                                                                               \
            (S).frames = (S).grown;                                                                                    \
            (S).cap = (S).new_cap;                                                                                     \
        }                                                                                                              \
    }                                                                                                                  \
    if ((S).depth < (S).cap) {                                                                                         \
        (S).frames[(S).depth].proc = (p);                                                                              \
        (S).frames[(S).depth].site = (s);                                                                              \
        (S).frames[(S).depth].gosubs = (S).main ? next_return_point : 0;                                               \
        (S).depth++;                                                                                                   \
    } else {                                                                                                           \
        (S).lost++;                                                                                                    \
    }

// A frame pushed from code that is not a procedure's start (mainerr.txt, an event's case).
#define qbr_ct_push_at(p, s)                                                                                           \
    {                                                                                                                  \
        qbr_ct_state &qbr_ct_e = qbr_ct;                                                                               \
        qbr_ct_push_to(qbr_ct_e, p, s)                                                                                 \
    }

// A procedure's end: its frame taken off S, and the caller's site back for its statement's next call. A lost frame is
// only uncounted. A procedure pops only after it pushed, so the stack is there, or the first allocation failed and
// depth is 0: then it reads qbr_ct_none.
static qbr_ct_frame qbr_ct_none[1];
#define qbr_ct_pop_from(S)                                                                                             \
    if ((S).lost) {                                                                                                    \
        (S).lost--;                                                                                                    \
    } else if ((S).depth) {                                                                                            \
        (S).depth--;                                                                                                   \
        (S).site = ((S).frames ? (S).frames : qbr_ct_none)[(S).depth].site;                                            \
    }

// A frame popped from code that is not a procedure's end (an event's case).
#define qbr_ct_pop()                                                                                                   \
    {                                                                                                                  \
        qbr_ct_state &qbr_ct_e = qbr_ct;                                                                               \
        qbr_ct_pop_from(qbr_ct_e)                                                                                      \
    }

// RESUME in a handler: the handler's frame taken off (design D5). It reads the state directly, so that the RESUME
// statement's line has no block of its own.
#define qbr_ct_resume()                                                                                                \
    if (!qbr_ct.lost && qbr_ct.depth && qbr_ct.frames[qbr_ct.depth - 1].proc == 1) {                                   \
        qbr_ct.depth--;                                                                                                \
        qbr_ct.site = qbr_ct.frames[qbr_ct.depth].site;                                                                \
    }

// RUN, and RESUME to a label of the main module: the code that was running is left, its C++ frames with it or not.
#define qbr_ct_reset()                                                                                                 \
    {                                                                                                                  \
        qbr_ct.depth = 0;                                                                                              \
        qbr_ct.lost = 0;                                                                                               \
    }

#define QBR_CT_NONE 0xFFFFFFFFu
#define QBR_CT_EVENT 0xFFFFFFFEu
#define QBR_CT_ROWS 24
#define QBR_CT_HEAD 16
#define QBR_CT_TAIL 7

// The text of the block, written without allocating: error 256 is reported with the stack nearly full.
static char qbr_ct_text[4096];
static size_t qbr_ct_len;

static void qbr_ct_put(const char *t) {
    while (*t && qbr_ct_len + 1 < sizeof(qbr_ct_text))
        qbr_ct_text[qbr_ct_len++] = *t++;
    qbr_ct_text[qbr_ct_len] = 0;
}

// The rows are counted first (write 0), then written with their number known, so that a long call stack shows its
// 16 innermost rows, a row of how many were left out, and its 7 outermost (spec compiler/call-trace).
struct qbr_ct_out {
    int write;
    uint32 n, total;
};

static void qbr_ct_row(qbr_ct_out &o, const char *text, uint32 proc, int gosub, uint32 file, uint32 line) {
    uint32 i = o.n++;
    if (!o.write)
        return;
    char num[64];
    if (o.total > QBR_CT_ROWS && i >= QBR_CT_HEAD && i < o.total - QBR_CT_TAIL) {
        if (i == QBR_CT_HEAD) {
            snprintf(num, sizeof(num), "\n  ... %u more ...", (unsigned)(o.total - QBR_CT_HEAD - QBR_CT_TAIL));
            qbr_ct_put(num);
        }
        return;
    }
    qbr_ct_put("\n  ");
    if (text) {
        qbr_ct_put(text);
        return;
    }
    qbr_ct_put(proc < qbr_ct_procs ? qbr_ct_names[proc] : "?");
    if (gosub)
        qbr_ct_put(", GOSUB");
    if (file == QBR_CT_EVENT) {
        qbr_ct_put(" (event)");
    } else if (file < qbr_ct_file_count) {
        qbr_ct_put(" (");
        qbr_ct_put(qbr_ct_files[file]);
        snprintf(num, sizeof(num), ":%u)", (unsigned)line);
        qbr_ct_put(num);
    }
}

// A row whose place is a site: not known, an event, or a file and line.
static void qbr_ct_site_row(qbr_ct_out &o, uint32 proc, int gosub, uint32 site) {
    if (site == 1)
        qbr_ct_row(o, NULL, proc, gosub, QBR_CT_EVENT, 0);
    else if (site >= 2 && site < qbr_ct_sites)
        qbr_ct_row(o, NULL, proc, gosub, qbr_ct_at[2 * site], qbr_ct_at[2 * site + 1]);
    else
        qbr_ct_row(o, NULL, proc, gosub, QBR_CT_NONE, 0);
}

// The rows, innermost first: each level's procedure, at the error's place for the innermost and at the call of the
// level above for the others, then its pending GOSUBs, newest first. The main module is the level below every
// frame, on its own thread only.
static void qbr_ct_walk(qbr_ct_out &o, uint32 file, uint32 line) {
    qbr_ct_state &s = qbr_ct;
    uint32 main = s.main ? 1 : 0;
    uint32 levels = s.depth + main;
    if (s.lost) {
        char lost[64];
        snprintf(lost, sizeof(lost), "... %u calls not recorded ...", (unsigned)s.lost);
        qbr_ct_row(o, lost, 0, 0, QBR_CT_NONE, 0);
        file = QBR_CT_NONE;
    }
    for (uint32 j = levels; j-- > 0;) {
        const qbr_ct_frame *f = j >= main ? &s.frames[j - main] : NULL;
        uint32 proc = f ? f->proc : 0;
        if (j == levels - 1)
            qbr_ct_row(o, NULL, proc, 0, file, line);
        else
            qbr_ct_site_row(o, proc, 0, s.frames[j + 1 - main].site);
        if (!main)
            continue;
        uint32 start = f ? f->gosubs : 0;
        uint32 end = j == levels - 1 ? next_return_point : s.frames[j + 1 - main].gosubs;
        if (end > next_return_point)
            end = next_return_point;
        // Return point 0 is an event's (events.rs), shown by the event's frame.
        for (uint32 g = end; g > start;) {
            uint32 id = return_point[--g];
            if (id && id < qbr_ct_gosub_count)
                qbr_ct_site_row(o, proc, 1, qbr_ct_gosub_at[id]);
        }
    }
}

// libqb's hook (design D6): the error's line in the main file, or its line in an included file and that file's name
// as the program's event checks give it (its base name). The text starts with a line end, so that libqb's
// "Continue?" follows it on a line of its own; with no row it is empty.
static const char *qbr_ct_hook(uint32_t line, uint32_t include_line, const char *include_file) {
    uint32 file = QBR_CT_NONE, at = 0;
    if (include_line) {
        for (uint32 i = 0; include_file && i < qbr_ct_file_count; i++) {
            if (i != qbr_ct_main_file && !strcmp(qbr_ct_bases[i], include_file)) {
                file = i;
                at = include_line;
                break;
            }
        }
    } else if (line) {
        file = qbr_ct_main_file;
        at = line;
    }
    qbr_ct_out o = {0, 0, 0};
    qbr_ct_walk(o, file, at);
    qbr_ct_len = 0;
    qbr_ct_text[0] = 0;
    if (!o.n)
        return qbr_ct_text;
    o.total = o.n;
    o.n = 0;
    o.write = 1;
    qbr_ct_put("\nCall stack:");
    qbr_ct_walk(o, file, at);
    return qbr_ct_text;
}

static const int32 qbr_ct_registered = (libqb_error_call_stack = qbr_ct_hook, 0);
