#![cfg_attr(not(test), no_std)]

extern crate r_efi;

mod framebuffer;
pub use framebuffer::*;
