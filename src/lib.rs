//! # `code_location`
//!
//! A library to automatically acquire a code location in a rust source code file.
//!
//! The library provides the [`code_location!()`] macro.
//! The macro expands to the [`CodeLocation`] struct
//! by combining the standard [`file!()`], [`line!()`] and [`column!()`] macros.
//!
//! [`code_location!()`]: ./macro.code_location.html
//! [`CodeLocation`]: ./struct.CodeLocation.html
//! [`file!()`]: https://doc.rust-lang.org/core/macro.file.html
//! [`line!()`]: https://doc.rust-lang.org/core/macro.line.html
//! [`column!()`]: https://doc.rust-lang.org/core/macro.column.html
//!
//! # Examples
//!
//! ```
//! use code_location::code_location;
//!
//! println!("I am printing from {}", code_location!());
//! assert_eq!(code_location!().to_string(), "src/lib.rs:7:12");
//! ```
//!
//! ```
//! use code_location::{code_location, CodeLocation};
//!
//! let code_location: CodeLocation = code_location!();
//! assert_eq!(code_location.file, "src/lib.rs");
//! assert_eq!(code_location.line, 6);
//! assert_eq!(code_location.column, 35);
//! ```
//!
//! # `#![no_std]`
//!
//! `code_location` only depends on the [`core`] library.
//!
//! ['core']: https://doc.rust-lang.org/core/index.html

#![no_std]

use core::fmt;

/// A CodeLocation type to represent a certain point in a source code text file.
///
/// # Examples
///
/// ```
/// use code_location::{code_location, CodeLocation};
///
/// let code_location: CodeLocation = code_location!();
/// assert_eq!(code_location.to_string(), "src/lib.rs:6:35");
/// assert_eq!(code_location.file, "src/lib.rs");
/// assert_eq!(code_location.line, 6);
/// assert_eq!(code_location.column, 35);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub struct CodeLocation {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// Expands to the [`CodeLocation`] on which it was invoked.
///
/// # Examples
///
/// ```
/// use code_location::code_location;
///
/// println!("I am printing from {}", code_location!());
/// assert_eq!(code_location!().to_string(), "src/lib.rs:7:12");
/// ```
///
/// [`CodeLocation`]: ./struct.CodeLocation.html
#[macro_export]
macro_rules! code_location {
    () => {
        $crate::CodeLocation {
            file: file!(),
            line: line!(),
            column: column!(),
        }
    };
}

#[cfg(test)]
mod tests;
