//! Seeded mutation test (spec `testing/compiler-tests`, design D6 of `m2-upstream-tests`): byte-level edits of
//! the corpus programs go through lexer, parser and `sema` without a panic and round-trip exactly.
//!
//! Deterministic: a fixed seed (`QB64RUST_MUTATE_SEED` overrides it) and a fixed number of mutants per program
//! (`QB64RUST_MUTATE_COUNT`, default 20). Each mutant has its own seed, derived from the run's seed, the program
//! and the mutant number, so a failure is reproduced from what it prints; the mutant's bytes are also written to
//! `target\mutate-failure.bas`.

use qb64rust_driver::frontend;
use qb64rust_syntax::tree::print;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};

const DEFAULT_SEED: u64 = 0x5EED_0B64_2026_1004;
const DEFAULT_COUNT: u64 = 20;

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

/// What went wrong with one mutant, if anything.
fn check(bytes: &[u8]) -> Option<String> {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let fe = frontend("mutant.bas", bytes.to_vec());
        print(&fe.parsed.main().green, bytes) == bytes
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
fn mutated_corpus_programs_go_through_the_front_end() {
    let seed = env_u64("QB64RUST_MUTATE_SEED", DEFAULT_SEED);
    let count = env_u64("QB64RUST_MUTATE_COUNT", DEFAULT_COUNT);
    let programs = corpus_programs();
    assert!(programs.len() >= 270, "found only {} corpus programs", programs.len());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let indexed: Vec<(usize, &PathBuf)> = programs.iter().enumerate().collect();
    let chunk = indexed.len().div_ceil(threads).max(1);
    // The first failure in program order: (program index, mutant, program name, problem, bytes).
    let failures: Vec<(usize, u64, String, String, Vec<u8>)> = std::thread::scope(|s| {
        let handles: Vec<_> = indexed
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    for &(i, path) in part {
                        let src = std::fs::read(path).unwrap();
                        for m in 0..count {
                            let bytes = mutate(&src, &mut Rng::new(mutant_seed(seed, i, m)));
                            if let Some(problem) = check(&bytes) {
                                let name = path.file_name().unwrap().to_string_lossy().to_string();
                                return Some((i, m, name, problem, bytes));
                            }
                        }
                    }
                    None
                })
            })
            .collect();
        handles.into_iter().filter_map(|h| h.join().unwrap()).collect()
    });
    if let Some((i, m, name, problem, bytes)) = failures.into_iter().min_by_key(|f| (f.0, f.1)) {
        let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/mutate-failure.bas");
        let _ = std::fs::write(&out, &bytes);
        panic!(
            "{problem}\nprogram {name} (corpus index {i}), mutant {m}, seed {seed}\nmutant written to {}\nreproduce: \
             QB64RUST_MUTATE_SEED={seed} cargo test -p qb64rust-driver --test mutate",
            out.display()
        );
    }
}
