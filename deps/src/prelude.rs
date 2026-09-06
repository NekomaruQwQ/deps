//! # Extra prelude for the Rust Standard Library
//!
//! This prelude contains the following re-exports:
//!
//! - Curated, commonly used types from the Rust Standard Library that are not
//!   included in the default Rust prelude ([`std::prelude::rust_2024`]). The
//!   most commonly requested ones include [`Rc`], [`RefCell`] and [`Arc`].
//! - Curated, commonly used modules from the Rust Standard Library. See below
//!   for details.
//! - Items from the [`std::io`] prelude, namely [`Read`], [`Write`], [`BufRead`]
//!   and [`Seek`].
//! - Items from the OS-specific preludes, namely [`std::os::windows::prelude`]
//!   and [`std::os::unix::prelude`], based on the target platform.
//! - The [`Itertools`] trait from the [`itertools`] crate.
//! - The syntax sugar traits from the [`tap`] crate.
//!
//! ## Re-exported Modules from the Standard Library
//!
//! Re-exporting modules from the Rust Standard Library makes it easier to use
//! the modules without having to import them explicitly or reference them with
//! fully qualified paths. However, doing so may cause name conflicts when crates
//! of the same name as the re-exported modules are depended on.
//!
//! Typically, if `std::xxx` is re-exported here, the shadowed  `xxx` crate is
//! considered either abandoned or not widely used. To be specific, the following
//! criteria are used to determine whether a crate is abandoned or not widely used.
//!
//! - If a crate has not been updated since Dec 6th, 2018, the release date of
//!   Rust Edition 2018, it is considered abandoned.
//! - If a crate has not reached `0.1.0` yet, or has been downloaded no more
//!   than 5000 times in total, it is considered not widely used.
//!
//! These criteria apply to the following crates:
//!
//! - The `array` crate was last updated to `0.0.1` over 11 years ago;
//! - The `cmp` crate has been downloaded only ~2200 times in total;
//! - The `fmt` crate was last updated to `0.1.0` about 8 years ago;
//! - The `fs` crate was last updated to `0.0.6`;
//! - The `future` crate does not exist;
//! - The `hash` crate was last updated to `0.3.0` over 11 years ago;
//! - The `io` crate was last updated to `0.0.2`;
//! - The `iter` crate is a binary crate;
//! - The `mem` crate was last updated to `0.5.0` about 9 years ago;
//! - The `net` crate was last updated to `0.0.2`;
//! - The `os` crate was last updated to `0.1.0` about 9 years ago;
//! - The `process` crate does not exist;
//! - The `slice` crate was last updated to `0.0.4` about 8 years ago;
//! - The `sync` crate was last updated to `0.1.0` about 8 years ago;
//! - The `thread` crate has been downloaded only ~500 times in total.
//!
//! Besides the above crates, the `env`, `ffi`, `ops`, and `ptr` crates are
//! considered not a risk of name conflict due to the following reasons:
//!
//! The `env` crate is literally an re-export of the [`std::env`] module, besides
//! two functions `env::set_var` and `env::remove_var`. Different from their
//! unsafe counterparts in the Rust Standard Library, these two functions are
//! safe on supported platforms, and return [`None`] on unsupported platforms.
//! Here we choose to re-export the `env` module from the Rust Standard Library,
//! preserving the standard semantics of the `set_var` and `remove_var` functions.
//!
//! The `ffi` crate only provides a `ffi::ffi` procedural macro, and therefore
//! does not cause a name conflict if imported using fully qualified
//! `use ::ffi::ffi`.
//!
//! The `ops` crate is a Rust implementation of the Operational Endpoints
//! Specification. Since it serves a niche purpose and is not related to the
//! `std::ops` module, we choose to re-export the commonly used [`std::ops`]
//! module for convenience.
//!
//! The `ptr` crate exposes smart pointers `ptr::Unique` and `ptr::Shared`. Since
//! [`std::boxed::Box`] and [`std::rc::Rc`] are the preferred smart pointers from
//! the Rust Standard Library, it is preferred here to re-export the `ptr` module
//! from the Rust Standard Library rather than holding the name for the `ptr` crate.
//!
//! Note that you can always use the following syntax to if a shadowed crate is
//! needed in your code:
//!
//! - Rename the crate in `Cargo.toml` using the `package` field. For example:
//!   ```toml
//!   [dependencies]
//!   safe_env = { package = "env", version = "latest" }
//!   ```
//! - Use the `extern crate` declaration to import the shadowed crate with a
//!   different name. For example, in your `lib.rs` or `main.rs`:
//!   ```no_run
//!   extern crate env as safe_env;
//!   ```
//! - Use the fully qualified path.
//!
//! Re-exporting the following modules are still under evaluation, and may be
//! added in the future:
//! - [`::std::num`]
//! - [`::std::time`]

pub use ::std::{
    array,   // crate last updated 0.0.1 over 11 years ago
    cmp,     // crate total downloads ~2200
    env,     // see above for details
    ffi,     // see above for details
    fmt,     // crate last updated 0.1.0 about 8 years ago
    fs,      // crate last updated 0.0.6
    future,  // crate does not exist
    hash,    // crate last updated 0.3.0 over 11 years ago
    io,      // crate last updated 0.0.2
    iter,    // crate is a binary crate
    mem,     // crate last updated 0.5.0 about 9 years ago
    net,     // crate last updated 0.0.2
    ops,     // see above for details
    os,      // crate last updated 0.1.0 about 9 years ago
    process, // crate does not exist
    ptr,     // see above for details
    slice,   // crate last updated 0.0.4 about 8 years ago
    sync,    // crate last updated 0.1.0 about 8 years ago, total downloads ~6000
    thread,  // crate total downloads ~500
};

pub use ::std::{
    borrow::{Borrow, BorrowMut, Cow, ToOwned},
    borrow::Cow::{Borrowed, Owned},
    cell::{Cell, LazyCell, OnceCell, RefCell},
    io::prelude::{BufRead, Read, Seek, Write},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

pub use ::std::collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    HashMap,
    HashSet,
    LinkedList,
    VecDeque,
    btree_map::Entry as BTreeMapEntry,
    hash_map::Entry as HashMapEntry,
};

#[cfg(target_family = "windows")]
pub use ::std::os::windows::prelude::*;

#[cfg(target_family = "unix")]
pub use ::std::os::unix::prelude::*;

pub use ::itertools::Itertools;
pub use ::tap::prelude::*;

/// Returns the default value of a type that implements the [`Default`]
/// trait.
///
/// This was once `std::default::default` behind the `default_free_fn`
/// feature gate, but that feature was later removed rather than
/// stabilized.
#[inline]
pub fn default<T: Default>() -> T { T::default() }
