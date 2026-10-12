// The screen-state check (tools/screen/README.md). This file is given to the link of a test program, of the old
// compiler or the new one, as an extra source file (-f:ExtraLinkerFlags=<this file>). It does nothing unless the
// program runs with one of two environment variables:
//
//   QB64_SCREEN_STATE=<file>  when the program exits, the state of the screen pages is written to the file
//   QB64_SCREEN_KEYS=<file>   the keys of the file are given to the program as if typed
//
// It reads libqb's own variables and calls keyboard_keydown, so it is compiled against the headers of the libqb
// it is linked with; libqb itself is not changed.

#include "cmem.h"
#include "graphics.h"
#include "keyboard.h"
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <fstream>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

extern int32_t pages; // libqb.cpp: the number of entries of `page`

namespace {

// ---------------------------------------------------------------------------------------------------------------
// The state file

// The number of the screen page an image is, or -1.
int page_of(const img_struct *p) {
    for (int i = 0; i < pages; i++)
        if (page[i] && &img[page[i]] == p)
            return i;
    return -1;
}

void put_place(FILE *f, const char *name, const img_struct *p) {
    int n = page_of(p);
    if (n < 0)
        fprintf(f, " %s image", name);
    else
        fprintf(f, " %s %d", name, n);
}

// A row as runs: "v" or "v*n", v in hex with `digits` digits.
void put_runs(FILE *f, const uint32_t *v, int n, int digits) {
    for (int i = 0; i < n;) {
        int j = i;
        while (j < n && v[j] == v[i])
            j++;
        if (j - i > 1)
            fprintf(f, " %0*X*%d", digits, v[i], j - i);
        else
            fprintf(f, " %0*X", digits, v[i]);
        i = j;
    }
    fputc('\n', f);
}

// One page or image: a header line, the palette (not for 32 bits), then its rows. A row equal to the one before
// is not written again: "= k" stands for k more rows like it.
void put_image(FILE *f, const char *what, const img_struct *im) {
    const int w = im->width, h = im->height;
    const bool wide = im->bytes_per_pixel == 4;
    fprintf(f, "%s mode %d %s %dx%d bits %d font %u cursor %d,%d %s", what, im->compatible_mode,
            im->text ? "text" : "graphics", w, h, im->bits_per_pixel, im->font, im->cursor_x, im->cursor_y,
            im->cursor_show ? "on" : "off");
    if (wide)
        fprintf(f, " color %08X background %08X\n", im->color, im->background_color);
    else
        fprintf(f, " color %u background %u\n", im->color, im->background_color);
    if (!wide && im->pal) {
        fprintf(f, "palette");
        put_runs(f, im->pal, 256, 8);
    }
    const size_t row_bytes = (size_t)w * (im->text ? 2 : im->bytes_per_pixel);
    std::vector<uint32_t> row(w);
    for (int y = 0; y < h; y++) {
        const uint8_t *p = im->offset + (size_t)y * row_bytes;
        if (im->text) {
            fprintf(f, "t \"");
            for (int x = 0; x < w; x++) {
                const uint8_t c = p[x * 2];
                if (c >= 0x20 && c < 0x7F && c != '\\' && c != '"')
                    fputc(c, f);
                else
                    fprintf(f, "\\x%02X", c);
                row[x] = p[x * 2 + 1];
            }
            fprintf(f, "\"\na");
            put_runs(f, row.data(), w, 2);
        } else {
            for (int x = 0; x < w; x++)
                row[x] = wide ? im->offset32[(size_t)y * w + x] : p[x];
            fprintf(f, "p");
            put_runs(f, row.data(), w, wide ? 8 : 2);
        }
        int same = 0;
        while (y + same + 1 < h && !memcmp(p, p + (size_t)(same + 1) * row_bytes, row_bytes))
            same++;
        if (same) {
            fprintf(f, "= %d\n", same);
            y += same;
        }
    }
}

void write_state() {
    const char *name = getenv("QB64_SCREEN_STATE");
    if (!name || !*name || !display_page)
        return;
    FILE *f = fopen(name, "wb");
    if (!f)
        return;
    fprintf(f, "screen-state 1\npages %d", pages);
    put_place(f, "display", display_page);
    put_place(f, "write", write_page);
    put_place(f, "read", read_page);
    fputc('\n', f);
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
    fprintf(f, "end\n");
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

struct Step {
    enum { Press, Wait, Repeat } kind;
    uint32_t value; // a key code, or milliseconds
};

// The steps of a keys file, one per line: "text <characters>", "key <name>", "wait <ms>", and as the last line
// "repeat <name>" (the key every 100 ms until the program exits: for END's "Press any key to continue").
// Returns the number of the first line that is none of these, or 0.
int read_steps(const std::string &script, std::vector<Step> &steps) {
    std::istringstream in(script);
    std::string line;
    bool repeats = false;
    for (int n = 1; std::getline(in, line); n++) {
        if (!line.empty() && line.back() == '\r')
            line.pop_back();
        if (line.empty() || line[0] == '#')
            continue;
        if (repeats)
            return n;
        const size_t sp = line.find(' ');
        const std::string verb = line.substr(0, sp), arg = sp == std::string::npos ? "" : line.substr(sp + 1);
        if (verb == "text" && !arg.empty()) {
            for (unsigned char c : arg) {
                if (c < 0x20 || c > 0x7E)
                    return n;
                steps.push_back({Step::Press, c});
            }
        } else if (verb == "key" || verb == "repeat") {
            uint32_t code = 0;
            for (const auto &k : key_names)
                if (arg == k.name)
                    code = k.code;
            if (!code)
                return n;
            repeats = verb == "repeat";
            steps.push_back({repeats ? Step::Repeat : Step::Press, code});
        } else if (verb == "wait" && !arg.empty() && arg.find_first_not_of("0123456789") == std::string::npos) {
            steps.push_back({Step::Wait, (uint32_t)atoi(arg.c_str())});
        } else {
            return n;
        }
    }
    return 0;
}

void sleep_ms(uint32_t ms) {
    std::this_thread::sleep_for(std::chrono::milliseconds(ms));
}

// A key goes in only when the INKEY$ buffer is empty: it holds 15 keys and drops what does not fit.
void press(uint32_t code) {
    while (cmem[0x41a] != cmem[0x41c])
        sleep_ms(1);
    keyboard_keydown(code);
    keyboard_keyup(code);
}

void feed(std::vector<Step> steps) {
    while (!cmem[0x41a]) // the runtime sets the buffer's head when it starts
        sleep_ms(1);
    for (const Step &s : steps) {
        if (s.kind == Step::Wait) {
            sleep_ms(s.value);
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
        std::thread(feed, std::move(steps)).detach();
    }
} start;

} // namespace
