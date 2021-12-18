/*! # Drew's core-foundation bindings for Rust

Provides Rust bindings for Apple Core Foundation.  This crate may be compared to
* [CoreFoundation-sys](https://crates.io/crates/CoreFoundation-sys)
* [core-foundation-sys](https://crates.io/crates/core-foundation-sys)
* [core-foundation](https://crates.io/crates/core-foundation)
* [RustKit](https://github.com/michaelwu/RustKit)

 Similar to my [other macOS libraries](https://github.com/drewcrawford/objr#objr-expanded-universe), this provides an elegant, performant, and largely safe wrapper
for macOS low-level libraries.

# Design notes:
* We use opaque types to model the underlying CFType (like `__CFString`, etc.)  Then the equivalent to `CFStringRef` is
  `&CFString`, a pointer type.
* Some optimizations are not yet implemented.  Among them, optimizations for static-time strings, inner pointers, zero-copy, etc.

# Implementation status

The following types are at least partially implemented.  Usually they contain "common APIs" or "the ones I use".
## Base types
* [base::CFOptionFlags]
* [base::CFTypeID]
* [base::CFRange]
* [base::CFType]
* [base::CFAllocator]

## Strings
* [base::CFString]

## Arrays
* [array::CFArray]

## Pointers
* [cell::StrongCell]

## Data
* [data::CFData]

## Dictionary
* [dictionary::CFDictionary]

## Error
* [error::CFError]

## Property list
* [property_list::MutabilityOptions]
* [property_list::Format]
* [property_list::CFPropertyList]


 */
mod base;
mod string;
mod cell;
mod data;
mod property_list;
mod error;
mod dictionary;
mod array;


pub use string::CFStringEncoding;
pub use base::{CFString,CFOptionFlags,CFTypeID,CFRange,CFType,CFAllocator,CFTypeAny};
pub use property_list::{MutabilityOptions,Format};
pub use data::CFData;
pub use property_list::CFPropertyList;
pub use base::CFTypeBehavior;
pub use dictionary::CFDictionary;
pub use array::CFArray;
pub use error::CFError;
pub use cell::StrongCell;