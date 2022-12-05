//! Documentation for this crate

#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::correctness)]
#![warn(deprecated)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::as_conversions)]
#![warn(clippy::style)]
#![warn(clippy::suspicious)]
#![allow(clippy::cargo)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_docs_in_private_items)]
#[allow(clippy::blanket_clippy_restriction_lints)]
#[allow(clippy::missing_inline_in_public_items)]
#[allow(clippy::print_stdout)]
#[allow(clippy::implicit_return)]
pub mod meet_setup;

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("library works");
    }
}
