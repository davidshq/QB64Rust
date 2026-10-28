//! Seeded mutation test (spec `testing/compiler-tests`, design D6 of `m2-upstream-tests`): byte-level edits of
//! the corpus programs go through lexer, parser and `sema` without a panic and round-trip exactly. A mutant that
//! `sema` accepts also goes through lowering and C++ generation (up to [`CODEGEN_MAX_LEN`]) for a debug build with
//! coverage counters, without a panic (`study\38` §3). Line-level edits (a line deleted, copied, swapped, or taken from
//! another corpus program) make mutants that are more often valid programs, so that more of them get that far.
//!
//! Deterministic: a fixed seed (`QB64RUST_MUTATE_SEED` overrides it) and a fixed number of mutants of each kind per
//! program (`QB64RUST_MUTATE_COUNT`, default 10). Each mutant has its own seed, derived from the run's seed, the
//! program and the mutant number, so a failure is reproduced from what it prints; the mutant's bytes are also
//! written to `target\mutate-failure.bas`.

use qb64rust_driver::{Resolved, STACK_SIZE, emit_ir_debug, frontend, lower};
use qb64rust_syntax::tree::print;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const DEFAULT_SEED: u64 = 0x5EED_0B64_2026_1004;
/// Ten of each kind: as many mutants as the twenty byte-level ones before C++ generation was added, at about 5 s
/// (each mutant per program of both kinds costs about 0.5 s of tier 1).
const DEFAULT_COUNT: u64 = 10;
/// A mutant larger than this skips C++ generation. Only `g12_game` (160 KB) is larger: its mutants took about 5 s
/// each to lower and emit here, most of the test's time, and add size rather than variety; the corpus suite builds
/// the program itself.
const CODEGEN_MAX_LEN: usize = 32 * 1024;

/// xorshift64*: small, fast, and the same on every platform.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Rng {
        // Mix the seed so that neighbouring seeds give unrelated streams; never zero.
        let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Rng((z ^ (z >> 31)) | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A number in `0..n` (`n` > 0).
    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % n as u64).expect("below n")
    }

    /// A number in `lo..=hi`.
    fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + self.below(hi - lo + 1)
    }

    /// A byte for an insertion: often one that matters to the lexer, otherwise any value.
    fn byte(&mut self) -> u8 {
        const SPECIAL: &[u8] = b"\x00\r\n\"'_:$&%!#(),.;=<>-+ \t";
        if self.below(2) == 0 {
            SPECIAL[self.below(SPECIAL.len())]
        } else {
            u8::try_from(self.next() & 0xFF).expect("one byte")
        }
    }
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// The seed of one mutant.
fn mutant_seed(seed: u64, program: usize, mutant: u64) -> u64 {
    seed ^ (program as u64).wrapping_mul(0x1000_0000_01B3) ^ mutant.wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
}

/// One to three edits: delete up to 16 bytes, insert 1-8 bytes, duplicate a range, swap two ranges.
fn mutate(src: &[u8], rng: &mut Rng) -> Vec<u8> {
    let mut s = src.to_vec();
    for _ in 0..rng.range(1, 3) {
        let len = s.len();
        match rng.below(4) {
            0 if len > 0 => {
                let at = rng.below(len);
                let n = rng.range(1, 16).min(len - at);
                s.drain(at..at + n);
            }
            1 | 0 => {
                let at = rng.below(len + 1);
                let bytes: Vec<u8> = (0..rng.range(1, 8)).map(|_| rng.byte()).collect();
                s.splice(at..at, bytes);
            }
            2 if len > 0 => {
                let at = rng.below(len);
                let n = rng.range(1, 16).min(len - at);
                let copy = s[at..at + n].to_vec();
                let to = rng.below(len + 1);
                s.splice(to..to, copy);
            }
            _ if len >= 2 => {
                // Cut points i <= j <= k <= l; the result is s[..i] s[k..l] s[j..k] s[i..j] s[l..].
                let i = rng.below(len);
                let j = (i + rng.range(1, 16)).min(len);
                let k = (j + rng.below(32)).min(len);
                let l = (k + rng.range(1, 16)).min(len);
                let mut out = s[..i].to_vec();
                out.extend_from_slice(&s[k..l]);
                out.extend_from_slice(&s[j..k]);
                out.extend_from_slice(&s[i..j]);
                out.extend_from_slice(&s[l..]);
                s = out;
            }
            _ => {}
        }
    }
    s
}

/// One to three edits of whole lines: delete one, copy one to another place, swap two, insert one of `donor`'s.
fn mutate_lines(src: &[u8], donor: &[u8], rng: &mut Rng) -> Vec<u8> {
    let mut lines: Vec<&[u8]> = src.split_inclusive(|&b| b == b'\n').collect();
    let donor: Vec<&[u8]> = donor.split_inclusive(|&b| b == b'\n').collect();
    for _ in 0..rng.range(1, 3) {
        let len = lines.len();
        match rng.below(4) {
            0 if len > 0 => {
                lines.remove(rng.below(len));
            }
            1 if len > 0 => {
                let line = lines[rng.below(len)];
                lines.insert(rng.below(len + 1), line);
            }
            2 if len >= 2 => {
                let (a, b) = (rng.below(len), rng.below(len));
                lines.swap(a, b);
            }
            _ if !donor.is_empty() => {
                let line = donor[rng.below(donor.len())];
                lines.insert(rng.below(len + 1), line);
            }
            _ => {}
        }
    }
    lines.concat()
}

fn corpus_programs() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bas")) {
                out.push(p);
            }
        }
    }
    let mut out = Vec::new();
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/corpus"),
        &mut out,
    );
    out.sort();
    out
}

/// What went wrong with one mutant, if anything; `emitted` counts the mutants that reached C++ generation.
fn check(bytes: &[u8], emitted: &AtomicUsize) -> Option<String> {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let fe = frontend("mutant.bas", bytes.to_vec());
        if print(&fe.parsed.main().green, bytes) != bytes {
            return false;
        }
        if !fe.has_errors() && bytes.len() <= CODEGEN_MAX_LEN {
            // One emission, the one that runs the most of the emitter: a debug build's, with coverage counters.
            emit_ir_debug(&fe, &lower(&fe), &Resolved::default(), true, None);
            emitted.fetch_add(1, Ordering::Relaxed);
        }
        true
    }));
    match result {
        Ok(true) => None,
        Ok(false) => Some("round trip failed".to_string()),
        Err(e) => Some(format!(
            "panic: {}",
            e.downcast_ref::<String>()
                .cloned()
                .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default()
        )),
    }
}

#[test]
fn mutated_corpus_programs_go_through_the_compiler() {
    let seed = env_u64("QB64RUST_MUTATE_SEED", DEFAULT_SEED);
    let count = env_u64("QB64RUST_MUTATE_COUNT", DEFAULT_COUNT);
    let programs = corpus_programs();
    assert!(programs.len() >= 270, "found only {} corpus programs", programs.len());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let sources: Vec<Vec<u8>> = programs.iter().map(|p| std::fs::read(p).unwrap()).collect();
    // Each thread takes the next mutant not yet taken, those of the largest programs first. One program's mutants
    // are not kept together: `g12_game` is a third of the corpus's bytes, and its mutants on one thread made the
    // run as long as that thread (as in `inputs.rs`).
    // Mutants 0..count edit bytes, count..2*count edit lines.
    let per_program = 2 * count;
    let mut order: Vec<(usize, u64)> = (0..programs.len())
        .flat_map(|i| (0..per_program).map(move |m| (i, m)))
        .collect();
    order.sort_by_key(|&(i, _)| std::cmp::Reverse(sources[i].len()));
    let next = AtomicUsize::new(0);
    let emitted = AtomicUsize::new(0);
    // A mutant's place in program order, and the first failing place found so far. A mutant after that place is
    // skipped: the one reported stays the first in program order, and a broken front end panics a few times, not
    // once per mutant.
    let place = |i: usize, m: u64| i * usize::try_from(per_program).unwrap() + usize::try_from(m).unwrap();
    let first = AtomicUsize::new(usize::MAX);
    // The failures: (program index, mutant, program name, problem, bytes); the first in program order is reported.
    let failures: Vec<(usize, u64, String, String, Vec<u8>)> = std::thread::scope(|s| {
        // The compiler's stack (`with_stack`): an overflow would abort the test binary with nothing reported.
        let handles: Vec<_> = (0..threads)
            .map(|_| {
                std::thread::Builder::new()
                    .stack_size(STACK_SIZE)
                    .spawn_scoped(s, || {
                        let mut part = Vec::new();
                        while let Some(&(i, m)) = order.get(next.fetch_add(1, Ordering::Relaxed)) {
                            if place(i, m) > first.load(Ordering::Relaxed) {
                                continue;
                            }
                            let mut rng = Rng::new(mutant_seed(seed, i, m));
                            let bytes = if m < count {
                                mutate(&sources[i], &mut rng)
                            } else {
                                let donor = (i + 1 + rng.below(sources.len() - 1)) % sources.len();
                                mutate_lines(&sources[i], &sources[donor], &mut rng)
                            };
                            if let Some(problem) = check(&bytes, &emitted) {
                                first.fetch_min(place(i, m), Ordering::Relaxed);
                                let name = programs[i].file_name().unwrap().to_string_lossy().to_string();
                                part.push((i, m, name, problem, bytes));
                            }
                        }
                        part
                    })
                    .expect("cannot start a mutation thread")
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    if let Some((i, m, name, problem, bytes)) = failures.into_iter().min_by_key(|f| (f.0, f.1)) {
        let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/mutate-failure.bas");
        let _ = std::fs::write(&out, &bytes);
        // The count is part of the repro: it decides which mutant numbers edit lines.
        let kind = if m < count { "byte-level" } else { "line-level" };
        panic!(
            "{problem}\nprogram {name} (corpus index {i}), mutant {m} ({kind}), seed {seed}, count {count}\nmutant \
             written to {}\nreproduce: QB64RUST_MUTATE_SEED={seed} QB64RUST_MUTATE_COUNT={count} cargo test -p \
             qb64rust-driver --test mutate",
            out.display()
        );
    }
    // About 43% reached it when this was written (seed and count by default); far fewer means the test no longer
    // tests what it says.
    let emitted = emitted.into_inner();
    assert!(
        emitted * 4 >= order.len(),
        "only {emitted} of {} mutants reached C++ generation",
        order.len()
    );
}
