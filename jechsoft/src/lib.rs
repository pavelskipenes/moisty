#![feature(iter_next_chunk)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]
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
#![warn(clippy::missing_const_for_fn)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::print_stdout)]
extern crate serde;
pub mod meet_setup;
pub mod uni_p;

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("library works");
    }
}
