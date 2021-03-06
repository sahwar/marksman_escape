//! A fastish HTML-encoding and HTML-decoding implementation in Rust.
//!
//! ### Fast…ish?
//!
//! The thing is that I only really compared the implementation against Python’s `html.encode` and
//! `html.decode` and Python’s numbers are so abysmal there’s nothing to compare…
//!
//! Anyway, here’s the (totally unscientific) numbers on Intel’s i7-4750HQ@2.00GHz and explanation
//! of them:
//!
//! ```ignore
//! test no_escape_bytes        ... bench:      1109 ns/iter (+/- 56) = 952 MB/s
//! test no_escape_bytes_filter ... bench:      2725 ns/iter (+/- 151) = 387 MB/s
//! test no_escape_chars        ... bench:      2695 ns/iter (+/- 144) = 391 MB/s
//! ```
//!
//! We have 3 benchmarks: two to check how fast can iterators be consumed and one to benchmark the
//! bare minimum filtering of the stream. These benchmarks serve as an anchor to compare with, so
//! it is clear how much overhead escaping and unescaping introduce.
//!
//! ```ignore
//! test escape_mixed            ... bench:      5595 ns/iter (+/- 129) = 175 MB/s
//! test escape_no_spec          ... bench:      6494 ns/iter (+/- 119) = 347 MB/s
//! test escape_spec_long        ... bench:      5515 ns/iter (+/- 157) = 117 MB/s
//! test escape_spec_short       ... bench:      4324 ns/iter (+/- 94)  = 150 MB/s
//!
//! test unescape_no_spec        ... bench:      7228 ns/iter (+/- 7)   = 242 MB/s
//! test unescape_spec_hex       ... bench:      3024 ns/iter (+/- 191) = 277 MB/s
//! test unescape_spec_named     ... bench:      8073 ns/iter (+/- 386) = 109 MB/s
//! test unescape_spec_num       ... bench:      2995 ns/iter (+/- 195) = 280 MB/s
//! ```
//!
//! Note, that both escape and unescape benchmarks test how fast the input is consumed, rather than
//! produced. They are likely to improve further as codegen for `Iterator`s is improved and my own
//! battles against LLVM are concluded.

pub use escape::{Escape};
pub use unescape::{Unescape};
pub use unescape_named::{get_named_ref};

mod escape;
mod unescape;
mod unescape_named;
