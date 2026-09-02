
pub mod prelude {
    /// Extra prelude for the Rust Standard Library.
    ///
    /// It is guaranteed that if `std::xxx` is re-exported here, the shadowed
    /// `xxx` crate is not popularly used. Note that you can always use the
    /// fully qualified path `::xxx` to reference the shadowed crate.
    ///
    /// Particularly, such crates satisfy at least one of the following conditions:
    /// - The crate is not a library crate.
    /// - The crate was released before Rust Edition 2018 (Dec 6th, 2018) and
    ///   has not been updated since then (as of Aug 2026).
    /// - The crate has not reached `0.1.0` yet (as of Aug 2026).
    /// - The crate has been downloaded no more than 5000 times in total so
    ///   far (as of Aug 2026).
    /// - The module in the Rust Standard Library is strictly preferred over
    ///   the crate in terms of functionality, performance, and/or safety, AND
    ///   re-exporting the std module brings significant convenience.
    /// - The crate does not overlap with the corresponding module in the Rust
    ///   Standard Library in terms of functionality AND re-exporting the std
    ///   module brings significant convenience.
    ///
    /// The last two conditions are the only ones that are not objectively
    /// verifiable, and therefore are evaluated with the utmost care and caution.
    ///
    /// The following modules and containing types are not re-exported here due
    /// to existing alternative implementations that may be preferred:
    /// - [`::std::collections`]
    ///
    /// The following modules can also be included here.
    /// - [`::std::num`]
    /// - [`::std::time`]
    pub mod std {
        pub use ::std::prelude::rust_2024::*;
        pub use ::std::{
            array,      // crate last updated 0.0.1 over 11 years ago
            cmp,        // crate total downloads ~2200
            ffi,        // crate is a proc macro crate
            fmt,        // crate last updated 0.1.0 about 8 years ago
            fs,         // crate last updated 0.0.6
            future,     // crate does not exist
            hash,       // crate last updated 0.3.0 over 11 years ago
            io,         // crate last updated 0.0.2
            iter,       // crate is a binary crate
            mem,        // crate last updated 0.5.0 about 9 years ago
            net,        // crate last updated 0.0.2
            ops,        // crate is not related
            os,         // crate last updated 0.1.0 about 9 years ago
            process,    // crate does not exist
            ptr,        // ::std::box::Box and ::std::rc::Rc are strictly preferred over ::ptr::Unique and ::ptr::Shared.
            slice,      // crate last updated 0.0.4 about 8 years ago
            sync,       // crate last updated 0.1.0 about 8 years ago, total downloads ~6000
            thread,     // total downloads ~500
        };

        pub use ::std::{
            borrow::*,
            borrow::Cow::{Borrowed, Owned},
            cell::{Cell, LazyCell, OnceCell, RefCell},
            iter::FromIterator,
            ops::{Deref, DerefMut},
            path::{Path, PathBuf},
            rc::Rc,
            str::FromStr,
            sync::Arc,
        };

        /// Returns the default value of a type that implements the [`Default`]
        /// trait.
        ///
        /// This was once `std::default::default` behind the `default_free_fn`
        /// feature gate, but that feature was later removed rather than
        /// stabilized.
        #[inline]
        pub fn default<T: Default>() -> T { T::default() }

        /// Inspection and manipulation of the process's environment.
        ///
        /// This is an identical re-export of the [`std::env`] module plus
        /// [`env::try_set_var`] and [`env::try_remove_var`] from the popular
        /// [`env`] crate.
        ///
        /// This module contains functions to inspect various aspects such as
        /// environment variables, process arguments, the current directory,
        /// and various other important directories.
        ///
        /// There are several functions and structs in this module that have
        /// a counterpart ending in `os`. Those ending in `os` will return an
        /// [`OsString`](std::ffi::OsString) and those without will return a
        /// [`String`].
        #[cfg(feature = "dep-env")]
        pub mod env {
            pub use ::std::env::*;
            pub use ::env::set_var    as try_set_var;
            pub use ::env::remove_var as try_remove_var;
        }

        #[cfg(not(feature = "dep-env"))]
        pub use ::std::env;
    }
}
