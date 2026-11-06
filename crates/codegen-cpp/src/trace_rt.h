// The run-time part of a value-trace build (`trace.rs`, design D3 of `value-trace`). It follows the tables
// `trace.rs` writes: the entries, the sites, the files, the aliases and the untraced stores.
#include <string>
#include <unordered_map>
#include <vector>
#include <algorithm>

// The kinds of an entry (`qbr_trc_kinds`).
#define QBR_TRC_INT 0
#define QBR_TRC_UINT 1
#define QBR_TRC_F32 2
#define QBR_TRC_F64 3
#define QBR_TRC_F80 4
#define QBR_TRC_STR 5

// mingw's own printf reads an 80-bit `long double`; the C runtime's does not.
#ifdef __MINGW32__
#define QBR_TRC_SNPRINTF __mingw_snprintf
#else
#define QBR_TRC_SNPRINTF snprintf
#endif

// A distinct value of an entry: how often it was recorded, the site that first recorded it, its place in the
// order of first records, and for a string keyed by its hash, its first bytes.
struct qbr_trc_value {
    uint64 count;
    uint32 site;
    uint32 order;
    std::string head;
};

struct qbr_trc_entry {
    // A number's key is its stored bytes. A string's is `S` and its bytes, or above 4096 bytes `H`, its length and
    // the FNV-1a 64 hash of its bytes, 8 bytes each.
    std::unordered_map<std::string, qbr_trc_value> values;
    std::string last;
    uint64 stores = 0, after_calls = 0;
    bool has_last = false, cut = false, ranged = false;
    int64 imin = 0, imax = 0;
    uint64 umin = 0, umax = 0;
    long double fmin = 0, fmax = 0;
};

// Never freed: the writer reads them at exit.
static qbr_trc_entry *qbr_trc_e = new qbr_trc_entry[QBR_TRC_ENTRIES + 1];
static size_t qbr_trc_limit = 0;

// The limit of distinct values per entry: 256, or the number `QB64RUST_TRACE_LIMIT` gives. Read once.
static size_t qbr_trc_get_limit() {
    const char *qbr_text = getenv("QB64RUST_TRACE_LIMIT");
    long long qbr_n = qbr_text ? atoll(qbr_text) : 0;
    return qbr_n > 0 ? (size_t)qbr_n : 256;
}

// Records `key` in entry `e`. An after-call record is dropped when the value is the entry's last recorded one, or
// its starting value (`zero`) when nothing was recorded yet. Gives whether the value was recorded.
static bool qbr_trc_put(int32 e, const std::string &key, int32 site, bool after, bool zero, const uint8 *head,
                        size_t head_len) {
    qbr_trc_entry &t = qbr_trc_e[e];
    if (after) {
        if (t.has_last ? t.last == key : zero)
            return false;
        t.after_calls++;
    } else {
        t.stores++;
    }
    t.last = key;
    t.has_last = true;
    auto found = t.values.find(key);
    if (found != t.values.end()) {
        found->second.count++;
        return true;
    }
    if (!qbr_trc_limit)
        qbr_trc_limit = qbr_trc_get_limit();
    if (t.values.size() >= qbr_trc_limit) {
        t.cut = true;
        return true;
    }
    qbr_trc_value v;
    v.count = 1;
    v.site = (uint32)site;
    v.order = (uint32)t.values.size();
    if (head)
        v.head.assign((const char *)head, head_len);
    t.values.emplace(key, v);
    return true;
}

static void qbr_trc_int(int32 e, int64 v, int32 site, bool after) {
    if (!qbr_trc_put(e, std::string((const char *)&v, 8), site, after, v == 0, NULL, 0))
        return;
    qbr_trc_entry &t = qbr_trc_e[e];
    if (!t.ranged) {
        t.imin = t.imax = v;
        t.ranged = true;
    } else if (v < t.imin) {
        t.imin = v;
    } else if (v > t.imax) {
        t.imax = v;
    }
}

static void qbr_trc_uint(int32 e, uint64 v, int32 site, bool after) {
    if (!qbr_trc_put(e, std::string((const char *)&v, 8), site, after, v == 0, NULL, 0))
        return;
    qbr_trc_entry &t = qbr_trc_e[e];
    if (!t.ranged) {
        t.umin = t.umax = v;
        t.ranged = true;
    } else if (v < t.umin) {
        t.umin = v;
    } else if (v > t.umax) {
        t.umax = v;
    }
}

// A float's key is its bytes, so -0 and 0 are two values and each NaN pattern is one of its own. A NaN leaves the
// range as it is.
static void qbr_trc_float(int32 e, const char *bytes, size_t size, long double v, int32 site, bool after) {
    bool zero = true;
    for (size_t i = 0; i < size; i++)
        zero = zero && !bytes[i];
    if (!qbr_trc_put(e, std::string(bytes, size), site, after, zero, NULL, 0) || v != v)
        return;
    qbr_trc_entry &t = qbr_trc_e[e];
    if (!t.ranged) {
        t.fmin = t.fmax = v;
        t.ranged = true;
    } else if (v < t.fmin) {
        t.fmin = v;
    } else if (v > t.fmax) {
        t.fmax = v;
    }
}

// The FNV-1a 64 hash of `len` bytes.
static uint64 qbr_trc_fnv(const char *bytes, size_t len) {
    uint64 h = 0xcbf29ce484222325ull;
    for (size_t i = 0; i < len; i++)
        h = (h ^ (uint8)bytes[i]) * 0x100000001b3ull;
    return h;
}

static void qbr_trc_bytes(int32 e, const uint8 *bytes, int64 len, bool fixed, int32 site, bool after) {
    if (len < 0)
        len = 0;
    std::string key;
    const uint8 *head = NULL;
    if (len > 4096) {
        uint64 n = (uint64)len, h = qbr_trc_fnv((const char *)bytes, (size_t)len);
        key.assign(1, 'H');
        key.append((const char *)&n, 8);
        key.append((const char *)&h, 8);
        head = bytes;
    } else {
        key.assign(1, 'S');
        key.append((const char *)bytes, (size_t)len);
    }
    // The starting value: an empty string, or the NUL bytes of a fixed-length one.
    bool zero = !fixed && !len;
    if (after && fixed) {
        zero = true;
        for (int64 i = 0; i < len; i++)
            zero = zero && !bytes[i];
    }
    if (!qbr_trc_put(e, key, site, after, zero, head, 200))
        return;
    qbr_trc_entry &t = qbr_trc_e[e];
    if (!t.ranged) {
        t.imin = t.imax = len;
        t.ranged = true;
    } else if (len < t.imin) {
        t.imin = len;
    } else if (len > t.imax) {
        t.imax = len;
    }
}

// The record calls of a store: `qbr_trc_<kind>(entry, value, site)`.
static inline void qbr_trc_i(int32 e, int64 v, int32 site) { qbr_trc_int(e, v, site, false); }
static inline void qbr_trc_u(int32 e, uint64 v, int32 site) { qbr_trc_uint(e, v, site, false); }
static inline void qbr_trc_f32(int32 e, float v, int32 site) { qbr_trc_float(e, (const char *)&v, 4, v, site, false); }
static inline void qbr_trc_f64(int32 e, double v, int32 site) { qbr_trc_float(e, (const char *)&v, 8, v, site, false); }
static inline void qbr_trc_f80(int32 e, long double v, int32 site) {
    qbr_trc_float(e, (const char *)&v, sizeof(v) < 10 ? sizeof(v) : 10, v, site, false);
}
static inline void qbr_trc_s(int32 e, qbs *v, int32 site) { qbr_trc_bytes(e, v->chr, v->len, v->fixed, site, false); }
// The `len` bytes of a fixed-length string at `v`.
static inline void qbr_trc_b(int32 e, const void *v, int64 len, int32 site) {
    qbr_trc_bytes(e, (const uint8 *)v, len, true, site, false);
}

// The same after a call that was given the variable by reference: recorded when the value changed.
static inline void qbr_trc_ai(int32 e, int64 v, int32 site) { qbr_trc_int(e, v, site, true); }
static inline void qbr_trc_au(int32 e, uint64 v, int32 site) { qbr_trc_uint(e, v, site, true); }
static inline void qbr_trc_af32(int32 e, float v, int32 site) { qbr_trc_float(e, (const char *)&v, 4, v, site, true); }
static inline void qbr_trc_af64(int32 e, double v, int32 site) { qbr_trc_float(e, (const char *)&v, 8, v, site, true); }
static inline void qbr_trc_af80(int32 e, long double v, int32 site) {
    qbr_trc_float(e, (const char *)&v, sizeof(v) < 10 ? sizeof(v) : 10, v, site, true);
}
static inline void qbr_trc_as(int32 e, qbs *v, int32 site) { qbr_trc_bytes(e, v->chr, v->len, v->fixed, site, true); }
static inline void qbr_trc_ab(int32 e, const void *v, int64 len, int32 site) {
    qbr_trc_bytes(e, (const uint8 *)v, len, true, site, true);
}

// Bytes as a JSON string of ASCII: a byte outside printable ASCII is `\u00XX`.
static void qbr_trc_json(FILE *f, const uint8 *bytes, size_t len) {
    fputc('"', f);
    for (size_t i = 0; i < len; i++) {
        uint8 c = bytes[i];
        if (c == '"' || c == '\\')
            fprintf(f, "\\%c", c);
        else if (c < 0x20 || c > 0x7e)
            fprintf(f, "\\u%04x", (unsigned)c);
        else
            fputc(c, f);
    }
    fputc('"', f);
}

// The text of a number from its key: an integer in decimal, a float so that it reads back to the same value.
static void qbr_trc_number(FILE *f, int32 kind, const char *bytes) {
    char text[64];
    if (kind == QBR_TRC_INT) {
        int64 v;
        memcpy(&v, bytes, 8);
        QBR_TRC_SNPRINTF(text, sizeof(text), "%lld", (long long)v);
    } else if (kind == QBR_TRC_UINT) {
        uint64 v;
        memcpy(&v, bytes, 8);
        QBR_TRC_SNPRINTF(text, sizeof(text), "%llu", (unsigned long long)v);
    } else if (kind == QBR_TRC_F32) {
        float v;
        memcpy(&v, bytes, 4);
        QBR_TRC_SNPRINTF(text, sizeof(text), "%.9g", (double)v);
    } else if (kind == QBR_TRC_F64) {
        double v;
        memcpy(&v, bytes, 8);
        QBR_TRC_SNPRINTF(text, sizeof(text), "%.17g", v);
    } else {
        long double v = 0;
        memcpy(&v, bytes, sizeof(v) < 10 ? sizeof(v) : 10);
        QBR_TRC_SNPRINTF(text, sizeof(text), "%.21Lg", v);
    }
    fputs(text, f);
}

// One end of an entry's range.
static void qbr_trc_bound(FILE *f, int32 kind, const qbr_trc_entry &t, bool max) {
    if (!t.ranged) {
        fputc('-', f);
    } else if (kind == QBR_TRC_INT || kind == QBR_TRC_STR) {
        fprintf(f, "%lld", (long long)(max ? t.imax : t.imin));
    } else if (kind == QBR_TRC_UINT) {
        fprintf(f, "%llu", (unsigned long long)(max ? t.umax : t.umin));
    } else {
        long double v = max ? t.fmax : t.fmin;
        if (kind == QBR_TRC_F32) {
            float x = (float)v;
            qbr_trc_number(f, kind, (const char *)&x);
        } else if (kind == QBR_TRC_F64) {
            double x = (double)v;
            qbr_trc_number(f, kind, (const char *)&x);
        } else {
            qbr_trc_number(f, kind, (const char *)&v);
        }
    }
}

// The record (design D6), appended to the file `QB64RUST_TRACE` names. Registered with `atexit` and
// `at_quick_exit`: every end of a libqb program reaches one of them.
static void qbr_trc_write() {
    const char *qbr_path = getenv("QB64RUST_TRACE");
    if (!qbr_path || !*qbr_path)
        return;
    FILE *f = fopen(qbr_path, "ab");
    if (!f)
        return;
    fprintf(f, "trace 1\n");
    for (int32 i = 0; i < QBR_TRC_FILES; i++)
        fprintf(f, "file\t%d\t%s\n", (int)i, qbr_trc_files[i]);
    for (int32 e = 0; e < QBR_TRC_ENTRIES; e++) {
        const qbr_trc_entry &t = qbr_trc_e[e];
        if (!t.stores && !t.after_calls)
            continue;
        int32 kind = qbr_trc_kinds[e];
        fprintf(f, "entry\t%d\t%s\t%llu\t%llu\t%llu\t%d\t", (int)e, qbr_trc_names[e], (unsigned long long)t.stores,
                (unsigned long long)t.after_calls, (unsigned long long)t.values.size(), (int)t.cut);
        qbr_trc_bound(f, kind, t, false);
        fputc('\t', f);
        qbr_trc_bound(f, kind, t, true);
        fputc('\n', f);
        // The values in the order they were first recorded.
        std::vector<const std::pair<const std::string, qbr_trc_value> *> values;
        for (const auto &v : t.values)
            values.push_back(&v);
        std::sort(values.begin(), values.end(),
                  [](const auto *a, const auto *b) { return a->second.order < b->second.order; });
        for (const auto *v : values) {
            const std::string &key = v->first;
            fprintf(f, "value\t%d\t%llu\t%u\t%u\t", (int)e, (unsigned long long)v->second.count,
                    (unsigned)qbr_trc_sites[2 * v->second.site], (unsigned)qbr_trc_sites[2 * v->second.site + 1]);
            if (kind != QBR_TRC_STR) {
                qbr_trc_number(f, kind, key.data());
            } else if (key[0] == 'H') {
                // The first 200 bytes, then the full length and the hash of all the bytes, which tell apart two
                // strings that share their first 200 bytes.
                uint64 n, h;
                memcpy(&n, key.data() + 1, 8);
                memcpy(&h, key.data() + 9, 8);
                qbr_trc_json(f, (const uint8 *)v->second.head.data(), v->second.head.size());
                fprintf(f, "\t%llu\t%016llx", (unsigned long long)n, (unsigned long long)h);
            } else if (key.size() - 1 > 200) {
                qbr_trc_json(f, (const uint8 *)key.data() + 1, 200);
                fprintf(f, "\t%llu\t%016llx", (unsigned long long)(key.size() - 1),
                        (unsigned long long)qbr_trc_fnv(key.data() + 1, key.size() - 1));
            } else {
                qbr_trc_json(f, (const uint8 *)key.data() + 1, key.size() - 1);
            }
            fputc('\n', f);
        }
    }
    for (int32 i = 0; i < QBR_TRC_ALIASES; i++)
        if (qbr_trc_al[i])
            fprintf(f, "alias\t%u\t%u\t%llu\t%s\n", (unsigned)qbr_trc_alias_at[2 * i],
                    (unsigned)qbr_trc_alias_at[2 * i + 1], (unsigned long long)qbr_trc_al[i], qbr_trc_alias[i]);
    for (int32 i = 0; i < QBR_TRC_UNTRACED; i++)
        if (qbr_trc_un[i])
            fprintf(f, "untraced\t%u\t%u\t%llu\t%s\n", (unsigned)qbr_trc_untraced_at[2 * i],
                    (unsigned)qbr_trc_untraced_at[2 * i + 1], (unsigned long long)qbr_trc_un[i], qbr_trc_untraced[i]);
    fprintf(f, "end\n");
    fclose(f);
}
static const int32 qbr_trc_registered = atexit(qbr_trc_write) | at_quick_exit(qbr_trc_write);
