// The screen-state check (tools/screen/README.md). This file is given to the link of a test program, of the old
// compiler or the new one, as an extra source file (-f:ExtraLinkerFlags=<this file>). It does nothing unless the
// program runs with one of two environment variables:
//
//   QB64_SCREEN_STATE=<file>  when the program exits, the state of the screen pages is written to the file
//   QB64_SCREEN_KEYS=<file>   the keys of the file are given to the program as if typed, its mouse steps as if
//                             the mouse moved over the window; its `state` steps write states while the program
//                             runs, before the one at exit
//   QB64_SCREEN_PROMPT_MS=<n> how long a `prompt` step waits for the program, and a mouse step for the window
//                             (default 30000)
//
// It reads libqb's own variables and calls keyboard_keydown and the Mouse_Queue... functions, so it is compiled
// against the headers of the libqb it is linked with; libqb itself is not changed.

#include "cmem.h"
#include "graphics.h"
#include "keyboard.h"
#include "mouse.h"
#include <chrono>
#include <cstdarg>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <mutex>
#include <set>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

extern int32_t pages; // libqb.cpp: the number of entries of `page`
// libqb.cpp: the last pixel of the screen in the window, set with each frame drawn; and the size of each font.
extern int32_t environment_2d__screen_x2, environment_2d__screen_y2;
extern int32_t *fontwidth, *fontheight;

namespace {

// ---------------------------------------------------------------------------------------------------------------
// The state file

// Where a state goes: a file, or the 64-bit FNV-1a hash of its bytes (a `digest` step).
struct Out {
    FILE *file = nullptr;
    uint64_t hash = 0xcbf29ce484222325ull;
    void put(const char *s, size_t n) {
        if (file) {
            fwrite(s, 1, n, file);
            return;
        }
        for (size_t i = 0; i < n; i++)
            hash = (hash ^ (uint8_t)s[i]) * 0x100000001b3ull;
    }
};

#if defined(__GNUC__)
__attribute__((format(printf, 2, 3)))
#endif
void outf(Out &o, const char *fmt, ...) {
    char buf[512];
    va_list args;
    va_start(args, fmt);
    const int n = vsnprintf(buf, sizeof buf, fmt, args);
    va_end(args);
    if (n > 0)
        o.put(buf, (size_t)n < sizeof buf ? (size_t)n : sizeof buf - 1);
}

void outc(Out &o, char c) {
    o.put(&c, 1);
}

// The number of the screen page an image is, or -1.
int page_of(const img_struct *p) {
    for (int i = 0; i < pages; i++)
        if (page[i] && &img[page[i]] == p)
            return i;
    return -1;
}

void put_place(Out &f, const char *name, const img_struct *p) {
    int n = page_of(p);
    if (n < 0)
        outf(f, " %s image", name);
    else
        outf(f, " %s %d", name, n);
}

// A row as runs: "v" or "v*n", v in hex with `digits` digits.
void put_runs(Out &f, const uint32_t *v, int n, int digits) {
    for (int i = 0; i < n;) {
        int j = i;
        while (j < n && v[j] == v[i])
            j++;
        if (j - i > 1)
            outf(f, " %0*X*%d", digits, v[i], j - i);
        else
            outf(f, " %0*X", digits, v[i]);
        i = j;
    }
    outc(f, '\n');
}

// One page or image: a header line, the palette (not for 32 bits), then its rows. A row equal to the one before
// is not written again: "= k" stands for k more rows like it.
void put_image(Out &f, const char *what, const img_struct *im) {
    const int w = im->width, h = im->height;
    const bool wide = im->bytes_per_pixel == 4;
    outf(f, "%s mode %d %s %dx%d bits %d font %u cursor %d,%d %s", what, im->compatible_mode,
         im->text ? "text" : "graphics", w, h, im->bits_per_pixel, im->font, im->cursor_x, im->cursor_y,
         im->cursor_show ? "on" : "off");
    if (wide)
        outf(f, " color %08X background %08X\n", im->color, im->background_color);
    else
        outf(f, " color %u background %u\n", im->color, im->background_color);
    if (!wide && im->pal) {
        outf(f, "palette");
        if (im->compatible_mode == 10) {
            // SCREEN 10: libqb sets entries 0 to 3 of the displayed page at every refresh, from entries 4 to 7 (the
            // mode's own palette) and the clock (the two phases of its blink, `GetTicks() & 512`). What they hold
            // depends on when the program ends, and on whether a refresh came at all, so they are written as 0.
            uint32_t pal[256];
            memcpy(pal, im->pal, sizeof pal);
            pal[0] = pal[1] = pal[2] = pal[3] = 0;
            put_runs(f, pal, 256, 8);
        } else {
            put_runs(f, im->pal, 256, 8);
        }
    }
    const size_t row_bytes = (size_t)w * (im->text ? 2 : im->bytes_per_pixel);
    std::vector<uint32_t> row(w);
    for (int y = 0; y < h; y++) {
        const uint8_t *p = im->offset + (size_t)y * row_bytes;
        if (im->text) {
            outf(f, "t \"");
            for (int x = 0; x < w; x++) {
                const uint8_t c = p[x * 2];
                if (c >= 0x20 && c < 0x7F && c != '\\' && c != '"')
                    outc(f, (char)c);
                else
                    outf(f, "\\x%02X", c);
                row[x] = p[x * 2 + 1];
            }
            outf(f, "\"\na");
            put_runs(f, row.data(), w, 2);
        } else {
            for (int x = 0; x < w; x++)
                row[x] = wide ? im->offset32[(size_t)y * w + x] : p[x];
            outf(f, "p");
            put_runs(f, row.data(), w, wide ? 8 : 2);
        }
        int same = 0;
        while (y + same + 1 < h && !memcmp(p, p + (size_t)(same + 1) * row_bytes, row_bytes))
            same++;
        if (same) {
            outf(f, "= %d\n", same);
            y += same;
        }
    }
}

// One state, from `screen-state 1` to `end`.
void put_state(Out &f) {
    outf(f, "screen-state 1\npages %d", pages);
    put_place(f, "display", display_page);
    put_place(f, "write", write_page);
    put_place(f, "read", read_page);
    outc(f, '\n');
    for (int i = 0; i < pages; i++)
        if (page[i] && img[page[i]].valid) {
            char what[32];
            snprintf(what, sizeof what, "page %d", i);
            put_image(f, what, &img[page[i]]);
        }
    // A destination or source that is no screen page (_DEST or _SOURCE of an image).
    if (write_page && page_of(write_page) < 0)
        put_image(f, read_page == write_page ? "image write read" : "image write", write_page);
    if (read_page && read_page != write_page && page_of(read_page) < 0)
        put_image(f, "image read", read_page);
    outf(f, "end\n");
}

// The states of one run go into one file: the first write starts it, every later one (a `state` step of the keys
// file, then the state at exit) is appended. The key thread and the exit may both write.
std::mutex state_lock;
bool state_started = false;

void write_state() {
    const char *name = getenv("QB64_SCREEN_STATE");
    if (!name || !*name || !display_page)
        return;
    std::lock_guard<std::mutex> hold(state_lock);
    FILE *f = fopen(name, state_started ? "ab" : "wb");
    if (!f)
        return;
    state_started = true;
    Out o;
    o.file = f;
    put_state(o);
    fclose(f);
}

// A `digest` step: one line `digest <16 hex digits>`, the FNV-1a 64 hash of the state `write_state` would write
// now. With QB64_SCREEN_DIGEST_AS_STATE=1 the state itself, to look at when a digest differs.
void write_digest() {
    const char *as_state = getenv("QB64_SCREEN_DIGEST_AS_STATE");
    if (as_state && !strcmp(as_state, "1")) {
        write_state();
        return;
    }
    const char *name = getenv("QB64_SCREEN_STATE");
    if (!name || !*name || !display_page)
        return;
    std::lock_guard<std::mutex> hold(state_lock);
    Out o;
    put_state(o);
    FILE *f = fopen(name, state_started ? "ab" : "wb");
    if (!f)
        return;
    state_started = true;
    fprintf(f, "digest %016llX\n", (unsigned long long)o.hash);
    fclose(f);
}

// ---------------------------------------------------------------------------------------------------------------
// The keys

const struct {
    const char *name;
    uint32_t code;
} key_names[] = {
    {"enter", 13},           {"esc", 27},             {"tab", 9},            {"backspace", 8},
    {"space", 32},           {"up", QBKC_UP},         {"down", QBKC_DOWN},   {"left", QBKC_LEFT},
    {"right", QBKC_RIGHT},   {"home", QBKC_HOME},     {"end", QBKC_END},     {"pageup", QBKC_PAGEUP},
    {"pagedown", QBKC_PAGEDOWN}, {"insert", QBKC_INSERT}, {"delete", QBKC_DELETE}, {"f1", QBKC_F1},
    {"f2", QBKC_F2},         {"f3", QBKC_F3},         {"f4", QBKC_F4},       {"f5", QBKC_F5},
    {"f6", QBKC_F6},         {"f7", QBKC_F7},         {"f8", QBKC_F8},       {"f9", QBKC_F9},
    {"f10", QBKC_F10},       {"f11", QBKC_F11},       {"f12", QBKC_F12},
};

// The modifier keys, for `hold` and `lift` only: the codes libqb's key callback passes for them (keyboard.cpp).
const struct {
    const char *name;
    uint32_t code;
} modifier_names[] = {
    {"lshift", VK + QBVK_LSHIFT}, {"rshift", VK + QBVK_RSHIFT}, {"lctrl", VK + QBVK_LCTRL},
    {"rctrl", VK + QBVK_RCTRL},   {"lalt", VK + QBVK_LALT},     {"ralt", VK + QBVK_RALT},
};

struct Step {
    enum { Press, Wait, Repeat, Prompt, State, Digest, MouseMove, MouseDown, MouseUp, Wheel, Rule, PlayUntil, Hold, Lift }
        kind;
    uint32_t value; // a key code, milliseconds, the line of a `prompt` or `play until`, or a mouse button
    // The pixel of a `mouse` step; the steps of a `wheel` in y; the place of a `rule` or `play until` in x and its
    // letter in y (-1: any letter).
    int32_t x = 0, y = 0;
    uint32_t line = 0;          // the line of a mouse step
    std::vector<uint32_t> keys; // the keys of a `rule`
};

// An integer, with a minus sign if `sign`: the whole of `s`, or false.
bool read_int(const std::string &s, bool sign, int32_t &v) {
    const size_t from = sign && !s.empty() && s[0] == '-' ? 1 : 0;
    if (s.size() == from || s.size() > from + 6 || s.find_first_not_of("0123456789", from) != std::string::npos)
        return false;
    v = atoi(s.c_str());
    return true;
}

uint32_t key_code(const std::string &name) {
    for (const auto &k : key_names)
        if (name == k.name)
            return k.code;
    return 0;
}

// A key of `hold` or `lift`: a name of `key` or a modifier.
uint32_t held_key_code(const std::string &name) {
    for (const auto &k : modifier_names)
        if (name == k.name)
            return k.code;
    return key_code(name);
}

// The keys of "text <characters>" (the rest of the line) and "key <name>" parts, one after the other, as a `rule`
// gives them: "key down key enter", "text ay", "key esc text y". False if `s` is not such parts.
bool read_keys(std::string s, std::vector<uint32_t> &keys) {
    while (!s.empty()) {
        if (s.compare(0, 5, "text ") == 0 && s.size() > 5) {
            for (unsigned char c : s.substr(5)) {
                if (c < 0x20 || c > 0x7E)
                    return false;
                keys.push_back(c);
            }
            return true;
        }
        if (s.compare(0, 4, "key ") != 0)
            return false;
        s = s.substr(4);
        const size_t sp = s.find(' ');
        const uint32_t code = key_code(s.substr(0, sp));
        if (!code)
            return false;
        keys.push_back(code);
        s = sp == std::string::npos ? "" : s.substr(sp + 1);
    }
    return !keys.empty();
}

// A place, "<n>" or "<n>:<letter>" (n from 0 to 255; no letter: any).
bool read_place(const std::string &s, int32_t &place, int32_t &letter) {
    const size_t colon = s.find(':');
    letter = -1;
    if (colon != std::string::npos) {
        if (s.size() != colon + 2 || (unsigned char)s[colon + 1] < 0x21 || (unsigned char)s[colon + 1] > 0x7E)
            return false;
        letter = (unsigned char)s[colon + 1];
    }
    return read_int(s.substr(0, colon), false, place) && place <= 255;
}

// The steps of a keys file, one per line: "text <characters>", "key <name>", "wait <ms>", "prompt", "state",
// "digest", the mouse steps "mouse <x> <y>" (a move to pixel x, y of the window's content), "press <button>" and
// "release <button>" (1 left, 2 right, 3 middle, at the last position), "wheel <n>" (n steps, negative up), the
// steps by places "rule <place> <keys>" and "play until <place>", a block "repeat <n>" … "end" (its steps n times),
// "hold <name>" and "lift <name>" (a key of `key` or a modifier, put down and left down until it is lifted), and as
// the last line "repeat <name>" (the key every 100 ms until the program exits: for END's "Press any key to
// continue"). A `hold` of a key already down, a `lift` of one that is not, and the `end` of a block that leaves
// other keys down than it found are not steps either. Returns the number of the first line that is none of these,
// or 0.
int read_steps(const std::string &script, std::vector<Step> &steps) {
    std::istringstream in(script);
    std::string line;
    bool repeats = false;
    struct Block {
        int32_t times;
        size_t first;            // its first step
        std::set<uint32_t> held; // the keys down where it starts
    };
    std::vector<Block> blocks; // the open `repeat <n>` blocks
    std::set<uint32_t> held;   // the keys down after the steps so far
    int n = 1;
    for (; std::getline(in, line); n++) {
        if (!line.empty() && line.back() == '\r')
            line.pop_back();
        if (line.empty() || line[0] == '#')
            continue;
        if (repeats)
            return n;
        const size_t sp = line.find(' ');
        const std::string verb = line.substr(0, sp), arg = sp == std::string::npos ? "" : line.substr(sp + 1);
        int32_t count = 0;
        if (verb == "text" && !arg.empty()) {
            for (unsigned char c : arg) {
                if (c < 0x20 || c > 0x7E)
                    return n;
                steps.push_back({Step::Press, c});
            }
        } else if (verb == "repeat" && read_int(arg, false, count)) {
            if (count < 1)
                return n;
            blocks.push_back({count, steps.size(), held});
        } else if (line == "end") {
            // A block run again must find the keys as it found them the first time.
            if (blocks.empty() || blocks.back().held != held)
                return n;
            const int32_t times = blocks.back().times;
            const size_t first = blocks.back().first;
            blocks.pop_back();
            const std::vector<Step> body(steps.begin() + (std::ptrdiff_t)first, steps.end());
            for (int32_t k = 1; k < times; k++)
                steps.insert(steps.end(), body.begin(), body.end());
        } else if (verb == "key" || verb == "repeat") {
            const uint32_t code = key_code(arg);
            if (!code || (verb == "repeat" && !blocks.empty()))
                return n;
            repeats = verb == "repeat";
            steps.push_back({repeats ? Step::Repeat : Step::Press, code});
        } else if (verb == "hold" || verb == "lift") {
            const uint32_t code = held_key_code(arg);
            // `insert` and `erase` each change the set only for a key in the right state.
            if (!code || (verb == "hold" ? !held.insert(code).second : held.erase(code) == 0))
                return n;
            steps.push_back({verb == "hold" ? Step::Hold : Step::Lift, code});
        } else if (verb == "rule") {
            const size_t sp2 = arg.find(' ');
            Step s{Step::Rule, 0};
            if (sp2 == std::string::npos || !read_place(arg.substr(0, sp2), s.x, s.y) ||
                !read_keys(arg.substr(sp2 + 1), s.keys))
                return n;
            steps.push_back(s);
        } else if (verb == "play") {
            Step s{Step::PlayUntil, (uint32_t)n};
            if (arg.compare(0, 6, "until ") != 0 || !read_place(arg.substr(6), s.x, s.y))
                return n;
            steps.push_back(s);
        } else if (line == "digest") {
            steps.push_back({Step::Digest, 0});
        } else if (verb == "wait" && !arg.empty() && arg.find_first_not_of("0123456789") == std::string::npos) {
            steps.push_back({Step::Wait, (uint32_t)atoi(arg.c_str())});
        } else if (verb == "mouse") {
            const size_t sp2 = arg.find(' ');
            Step s{Step::MouseMove, 0};
            if (sp2 == std::string::npos || !read_int(arg.substr(0, sp2), false, s.x) ||
                !read_int(arg.substr(sp2 + 1), false, s.y))
                return n;
            s.line = (uint32_t)n;
            steps.push_back(s);
        } else if (verb == "press" || verb == "release") {
            int32_t b = 0;
            if (!read_int(arg, false, b) || b < 1 || b > 3)
                return n;
            steps.push_back({verb == "press" ? Step::MouseDown : Step::MouseUp, (uint32_t)b});
            steps.back().line = (uint32_t)n;
        } else if (verb == "wheel") {
            Step s{Step::Wheel, 0};
            if (!read_int(arg, true, s.y) || s.y == 0)
                return n;
            s.line = (uint32_t)n;
            steps.push_back(s);
        } else if (line == "prompt") {
            steps.push_back({Step::Prompt, (uint32_t)n});
        } else if (line == "state") {
            steps.push_back({Step::State, 0});
        } else {
            return n;
        }
    }
    // A block that is never closed: the line after the last.
    return blocks.empty() ? 0 : n;
}

void sleep_ms(uint32_t ms) {
    std::this_thread::sleep_for(std::chrono::milliseconds(ms));
}

// A key goes in only when the INKEY$ buffer is empty: it holds 15 keys and drops what does not fit. A modifier
// (a code from VK up) writes nothing there and does not wait.
void wait_for_buffer(uint32_t code) {
    if (code >= VK)
        return;
    while (cmem[0x41a] != cmem[0x41c])
        sleep_ms(1);
}

void press(uint32_t code) {
    wait_for_buffer(code);
    keyboard_keydown(code);
    keyboard_keyup(code);
}

// A program announces that it waits for a key by counting in byte 0x4F0 of conventional memory, which libqb does
// not use: `DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG`, right before its read. The k-th `prompt`
// of the keys file waits for the k-th announcement: then everything the program draws before that read is drawn.
// Before the count it may write the place of the wait (1 to 255) to byte 0x4F1 and a letter to 0x4F2 (design D4 of
// cws-whole-game), which `rule` and `play until` read; a program that writes neither waits at place 0.
const int announced_at = 0x4F0, place_at = 0x4F1, letter_at = 0x4F2;

// Ends the program from the key thread: the state as it is, a message, and an exit code of the check's own. The
// message is also the last line of the state file: libqb points stderr at the console the program was started
// from (`AttachConsole`), so a test that reads the program's stderr through a pipe does not see it. `quick_exit`
// skips the `atexit` handlers (libqb's, run from this thread) but runs the `at_quick_exit` ones: a coverage build
// (`qbr_cov_write`) still writes its counts.
[[noreturn]] void stop(int code, const char *keys, uint32_t line, const char *why) {
    write_state();
    if (const char *name = getenv("QB64_SCREEN_STATE"))
        if (*name) {
            std::lock_guard<std::mutex> hold(state_lock);
            if (FILE *f = fopen(name, state_started ? "ab" : "wb")) {
                fprintf(f, "stopped %s line %u: %s\n", keys, line, why);
                fclose(f);
            }
        }
    fprintf(stderr, "screen_state: %s line %u: %s\n", keys, line, why);
    fflush(stderr);
    quick_exit(code);
}

void prompt(const char *keys, uint32_t line, uint8_t expected, uint32_t limit_ms) {
    const auto until = std::chrono::steady_clock::now() + std::chrono::milliseconds(limit_ms);
    for (;;) {
        const uint8_t ahead = (uint8_t)(cmem[announced_at] - expected);
        if (ahead == 0)
            return;
        if (ahead < 128)
            stop(125, keys, line, "the program announced more waits than the keys file has prompts");
        if (std::chrono::steady_clock::now() > until)
            stop(124, keys, line, "no prompt in time");
        sleep_ms(1);
    }
}

// Whether libqb has drawn a frame of the displayed page at the page's own size, which is when it sets how a pixel
// of the window maps to the screen. Before that `_MOUSEX` and `_MOUSEY` give -1 (libqb's screen size is still 0) or
// map by the size of the screen before the last `SCREEN`: a program that reads its events right after `SCREEN`
// would see that (found while task 4.3 of m3-images-fonts-mouse recorded its programs).
// It reads the page from this thread without a lock: a `SCREEN` run by the program while a mouse step waits can free
// the page under it and crash the test program. No mouse program changes the screen after its `prompt`; one that
// does must announce a wait after the change and put its mouse steps after that `prompt`.
bool window_shows_screen() {
    const img_struct *p = display_page;
    if (!p)
        return false;
    int32_t w = p->width, h = p->height;
    if (p->text) {
        w *= fontwidth[p->font];
        h *= fontheight[p->font];
    }
    return environment_2d__screen_x2 == w - 1 && environment_2d__screen_y2 == h - 1;
}

// A place as the keys file writes it: "9", or "14:M".
std::string place_name(int32_t place, int32_t letter) {
    std::string s = std::to_string(place);
    if (letter > 0x20 && letter < 0x7F)
        s += ':', s += (char)letter;
    else if (letter > 0)
        s += ":" + std::to_string(letter);
    return s;
}

// `play until <place>`: each wait the program announces is answered by the rules (`rules`: place, letter or -1,
// keys; the last set first) until one at `place` and `letter` (-1: any). A rule's first key answers the wait it
// matched, its other keys the waits after it, before any rule is looked at again or the place is checked. The wait
// at the place then counts as a `prompt`: the steps after `play until` answer it.
void play_until(const char *keys, const Step &s, uint8_t &prompts, uint32_t limit_ms,
                const std::vector<Step> &rules, std::vector<uint32_t> &pending) {
    for (;;) {
        prompt(keys, s.value, ++prompts, limit_ms);
        const int32_t place = cmem[place_at], letter = cmem[letter_at];
        if (!pending.empty()) {
            const uint32_t k = pending.front();
            pending.erase(pending.begin());
            press(k);
            continue;
        }
        if (place == s.x && (s.y < 0 || s.y == letter))
            return;
        const Step *rule = nullptr;
        for (auto r = rules.rbegin(); r != rules.rend() && !rule; ++r)
            if (r->x == place && r->y == letter)
                rule = &*r;
        for (auto r = rules.rbegin(); r != rules.rend() && !rule; ++r)
            if (r->x == place && r->y < 0)
                rule = &*r;
        if (!rule) {
            const std::string why = "no rule for the wait at place " + place_name(place, letter);
            stop(125, keys, s.value, why.c_str());
        }
        pending.assign(rule->keys.begin() + 1, rule->keys.end());
        press(rule->keys.front());
    }
}

void feed(std::vector<Step> steps, std::string keys) {
    uint32_t limit_ms = 30000;
    if (const char *ms = getenv("QB64_SCREEN_PROMPT_MS"))
        if (*ms)
            limit_ms = (uint32_t)atoi(ms);
    while (!cmem[0x41a]) // the runtime sets the buffer's head when it starts
        sleep_ms(1);
    // Counted from 0, not from the byte as it is now: the program may announce its first wait before this thread
    // wakes, and cmem starts as zeros.
    uint8_t prompts = 0;
    double mx = 0, my = 0; // the last position of a `mouse` step
    std::vector<Step> rules;       // the `rule` steps met so far
    std::vector<uint32_t> pending; // the keys of a rule still to give, one per wait
    for (const Step &s : steps) {
        // A mouse step goes into libqb's queue of mouse events at once, as the window's callbacks put one there:
        // the queue holds 65536 events, so nothing waits as a key does. The queue is a single-producer ring buffer
        // and this thread a second writer beside the window's callbacks: a real cursor event at the same moment can
        // lose or overwrite one. Those callbacks drop moves and scrolls while the window has no focus or the cursor
        // is outside it, so a test is safe with the real cursor off the window (design D4 of m3-images-fonts-mouse).
        // Each waits until the window shows the displayed page, so the program maps the event by that page.
        if (s.line) {
            const auto until = std::chrono::steady_clock::now() + std::chrono::milliseconds(limit_ms);
            while (!window_shows_screen()) {
                if (std::chrono::steady_clock::now() > until)
                    stop(124, keys.c_str(), s.line, "the window never showed the screen");
                sleep_ms(1);
            }
        }
        if (s.kind == Step::MouseMove) {
            mx = s.x;
            my = s.y;
            Mouse_QueuePositionEvent(mx, my, GLUTEnum_MouseCursorMode::Normal);
        } else if (s.kind == Step::MouseDown) {
            Mouse_QueueButtonDownEvent((int)s.value, mx, my);
        } else if (s.kind == Step::MouseUp) {
            Mouse_QueueButtonUpEvent((int)s.value, mx, my);
        } else if (s.kind == Step::Wheel) {
            // libqb's yOffset is GLFW's (positive up); _MOUSEWHEEL is positive down.
            Mouse_QueueScrollEvent(mx, my, 0, -(double)s.y);
        } else if (s.kind == Step::Wait) {
            sleep_ms(s.value);
        } else if (s.kind == Step::Prompt) {
            prompt(keys.c_str(), s.value, ++prompts, limit_ms);
        } else if (s.kind == Step::State) {
            write_state();
        } else if (s.kind == Step::Digest) {
            write_digest();
        } else if (s.kind == Step::Rule) {
            // A rule for the same place and letter replaces the earlier one.
            for (auto r = rules.begin(); r != rules.end();)
                r = r->x == s.x && r->y == s.y ? rules.erase(r) : r + 1;
            rules.push_back(s);
        } else if (s.kind == Step::PlayUntil) {
            play_until(keys.c_str(), s, prompts, limit_ms, rules, pending);
        } else if (s.kind == Step::Hold) {
            // A key still down when the program exits stays down: libqb ends with the process.
            wait_for_buffer(s.value);
            keyboard_keydown(s.value);
        } else if (s.kind == Step::Lift) {
            keyboard_keyup(s.value);
        } else {
            do {
                press(s.value);
                if (s.kind == Step::Repeat)
                    sleep_ms(100);
            } while (s.kind == Step::Repeat);
        }
    }
}

struct Start {
    Start() {
        atexit(write_state);
        const char *name = getenv("QB64_SCREEN_KEYS");
        if (!name || !*name)
            return;
        std::ifstream in(name, std::ios::binary);
        std::stringstream text;
        text << in.rdbuf();
        std::vector<Step> steps;
        const int bad = in ? read_steps(text.str(), steps) : -1;
        if (bad) {
            if (bad < 0)
                fprintf(stderr, "screen_state: cannot read %s\n", name);
            else
                fprintf(stderr, "screen_state: %s line %d: not a step\n", name, bad);
            _Exit(125);
        }
        std::thread(feed, std::move(steps), std::string(name)).detach();
    }
} start;

} // namespace
