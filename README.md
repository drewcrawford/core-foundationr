# Drew's core-foundation bindings for Rust

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
* `CFOptionFlags`
* `CFTypeID`
* `CFRange`
* `CFType`
* `CFAllocator`

## Strings
* `CFString`

## Arrays
* `CFArray`

## Pointers
* `StrongCell`

## Data
* `CFData`

## Dictionary
* `CFDictionary`

## Error
* `CFError`

## Property list
* `MutabilityOptions`
* `Format`
* `CFPropertyList`



